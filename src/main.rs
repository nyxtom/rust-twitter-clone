#![feature(local_key_cell_methods)]
use std::{future::Future, pin::Pin, time::Duration};

use async_redis_session::RedisSessionStore;
use mongodb::{options::ClientOptions, Client};
use registry::State;
use serde::{Deserialize, Serialize};
use tide::log::LogMiddleware;
use tide_flash::{cookies::CookieStore, FlashMiddleware};

mod registry;
mod repos;
mod request_ext;
mod route_ext;
mod routes;
mod templates;

mod prelude {
    pub use crate::request_ext::*;
    pub use crate::route_ext::*;
    pub use tide_flash::ext::*;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    username: String,
    uid: String,
    exp: usize,
    totp_enabled: bool,
    totp_attempt: usize,
    totp: Option<usize>,
}

fn no_store<'a>(
    req: tide::Request<State>,
    next: tide::Next<'a, State>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    use tide::http::cache::{CacheControl, CacheDirective};
    Box::pin(async {
        let mut res = next.run(req).await;

        if let None = res.header("Cache-Control") {
            let mut header = CacheControl::new();
            header.push(CacheDirective::NoStore);
            header.push(CacheDirective::MaxAge(Duration::from_secs(0)));

            res.insert_header(header.name(), header.value());
        }
        Ok(res)
    })
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    // configure mongodb client options
    let mongodb_url = std::env::var("MONGODB_URI")?;
    let app_name = std::env::var("APP_NAME")?;
    let mut client_options = ClientOptions::parse(mongodb_url).await?;
    client_options.app_name = Some(app_name);
    let client = Client::with_options(client_options)?;

    // setup tide app with client
    let mut app = tide::with_state(State::new(client));
    dotenv::dotenv().ok();
    tide::log::start();

    app.with(no_store);
    app.with(LogMiddleware::new());

    // configure openid connect and session middleware
    let session_secret = std::env::var("SESSION_SECRET")?;
    let redis_url = std::env::var("REDIS_URL")?;
    app.with(tide::sessions::SessionMiddleware::new(
        RedisSessionStore::new(redis_url)?,
        session_secret.as_bytes(),
    ));
    app.with(FlashMiddleware::new(CookieStore::default()));
    routes::configure(&mut app);

    let host = std::env::var("HOST").unwrap_or(String::from("0.0.0.0"));
    let port: u16 = std::env::var("PORT")?.parse()?;
    app.listen((host, port)).await?;

    Ok(())
}

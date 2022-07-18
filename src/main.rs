#![feature(local_key_cell_methods)]
use std::time::Duration;

use async_redis_session::RedisSessionStore;
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
    uid: u64,
    exp: usize,
    totp_enabled: bool,
    totp_attempt: usize,
    totp: Option<usize>,
}

async fn no_store(req: tide::Request<State>, next: tide::Next<'_, State>) -> tide::Result {
    use tide::http::cache::{CacheControl, CacheDirective};
    let mut res = next.run(req).await;

    if let None = res.header("Cache-Control") {
        let mut header = CacheControl::new();
        header.push(CacheDirective::NoStore);
        header.push(CacheDirective::MaxAge(Duration::from_secs(0)));

        res.insert_header(header.name(), header.value());
    }
    Ok(res)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::with_state(State::new());
    dotenv::dotenv().ok();
    tide::log::start();

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

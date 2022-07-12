use async_redis_session::RedisSessionStore;
use registry::State;
use serde::{Deserialize, Serialize};
use tide::log::LogMiddleware;
use tide_flash::{cookies::CookieStore, FlashMiddleware};

mod registry;
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
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::with_state(State::new());
    dotenv::dotenv().ok();
    env_logger::init();

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

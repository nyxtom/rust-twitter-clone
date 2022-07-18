# Full Stack Rust Web App - Twitter Clone

This is a programming exercise in Rust to create a simple Twitter clone using the [tide](https://docs.rs/tide) web framework, [async-std](https://docs.rs/async-std), [http-types](https://docs.rs/http-types), [tailwindcss](https://tailwindcss.com), [serde](https://docs.rs/serde) and [mongodb](https://github.com/mongodb/mongo-rust-driver). Since this is a full stack web application, it will also require a deployment setup and I've chosen to use [Digital Ocean](https://www.digitalocean.com/) and the [Digital Ocean App Platform](https://www.digitalocean.com/products/app-platform) to deploy with [Docker Containers](https://www.docker.com/). We will make use of [doctl](https://docs.digitalocean.com/reference/doctl/) when interacting with Digital Ocean to generate a simple `spec.yml` and make it so we can regularly update our project with continuous deployment through git.

## Dependencies

To get started we are going to need the following dependencies (after initializing the project with `cargo init`).

```toml
[package]
name = "twitter-clone"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
async-std = { version = "1.12.0", features = ["attributes"] }
handlebars = "4.3.1"
serde = { version = "1.0.138", features = ["derive"] }
tide = { version = "0.17.0-beta.1", features = ["sessions", "h1-server", "cookies"] }
tide-flash = { version = "0.1.1-beta.1" }
env_logger = "0.9.0"
dotenv = "0.15.0"
time = { version = "0.2.6", default-features = false, features = ["std"] }
serde_json = "1.0.82"
async-redis-session = "0.2.2"
libreauth = { version = "0.15.0", features = ["oath-uri"] }
qrcode = "0.12.0"
async-trait = "0.1.56"
validator = { version = "0.15.0", features = ["derive"] }
uuid = { version = "1.1.2", features = ["serde", "v4"] }
```

This will give us the basics we need to build a simple web application using an async runtime ([async-std](https://docs.rs/async-std)), a web framework ([tide](https://docs.rs/tide)), a templating library ([handlebars](https://handlebarsjs.org)), and JSON support ([serde](https://docs.rs/serde)). A lot of the other dependencies here to are to fill in for logging, [libreauth](https://docs.rs/libreauth) for two-factor totp authentication/token generation/validation, [validator](https://docs.rs/validator) for serde style derive validation on forms via structs, [uuid](https://docs.rs/uuid) for unique id generation, [qrcode](https://docs.rs/qrcode) to generate a qr code for the two-factor token uri, [async-redis-session](https://docs.rs/async-redis-session) to support the [redis](https://redis.io) based backend for session middleware in tide.

## 





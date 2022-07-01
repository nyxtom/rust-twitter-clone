# Full Stack Rust Web App - Twitter Clone

This is a programming exercise in Rust to create a simple Twitter clone using the [tide](https://docs.rs/tide) web framework, [async-std](https://docs.rs/async-std), [http-types](https://docs.rs/http-types), [tailwindcss](https://tailwindcss.com), [serde](https://docs.rs/serde) and [mongodb](https://github.com/mongodb/mongo-rust-driver). Since this is a full stack web application, it will also require a deployment setup and I've chosen to use [Digital Ocean](https://www.digitalocean.com/) and the [Digital Ocean App Platform](https://www.digitalocean.com/products/app-platform) to deploy with [Docker Containers](https://www.docker.com/). We will make use of [doctl](https://docs.digitalocean.com/reference/doctl/) when interacting with Digital Ocean to generate a simple `spec.yml` and make it so we can regularly update our project with continuous deployment via git.

## Dependencies

To get started we are going to need the following dependencies (after initializing the project with `cargo init`).

```toml
[package]
name = "twitter-clone"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

use handlebars::Handlebars;
use mongodb::{Client, Collection};
use serde::Serialize;

use crate::repos::user::User;

#[derive(Clone)]
pub struct State {
    pub registry: Handlebars<'static>,
    pub client: Client,
    db_name: String,
}

impl State {
    pub fn new(client: Client) -> State {
        let db_name = std::env::var("DB_NAME").expect("Must specify a database name");
        let mut state = State {
            registry: Handlebars::new(),
            client,
            db_name,
        };
        state.register_template("index.html", "static/index.html");
        state.register_template("login.html", "static/login.html");
        state.register_template("otp.html", "static/otp.html");
        state.register_template("2fa.html", "static/2fa.html");
        state.register_template("settings.html", "static/settings.html");
        state.register_template("register.html", "static/register.html");
        state
    }

    pub fn register_template(&mut self, name: &str, path: &str) {
        self.registry.register_template_file(name, path).unwrap();
    }

    pub fn db<T: Serialize>(&self, collection_name: &str) -> Collection<T> {
        self.client
            .database(&self.db_name)
            .collection::<T>(collection_name)
    }

    pub fn users(&self) -> Collection<User> {
        self.db::<User>("users")
    }

    pub fn render<T: Serialize>(
        &self,
        name: &str,
        data: &T,
    ) -> Result<String, handlebars::RenderError> {
        self.registry.render(name, data)
    }
}

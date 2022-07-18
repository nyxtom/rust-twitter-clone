use std::cell::RefCell;

use handlebars::Handlebars;
use serde::Serialize;

use crate::repos::{user::User, MemoryStore};

thread_local! {
    pub static USERS: RefCell<MemoryStore<User>> = RefCell::new(MemoryStore::new());
}

#[derive(Clone)]
pub struct State {
    pub registry: Handlebars<'static>,
}

impl State {
    pub fn new() -> State {
        let mut state = State {
            registry: Handlebars::new(),
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

    pub fn render<T: Serialize>(
        &self,
        name: &str,
        data: &T,
    ) -> Result<String, handlebars::RenderError> {
        self.registry.render(name, data)
    }
}

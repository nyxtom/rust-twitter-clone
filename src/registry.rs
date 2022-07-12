use handlebars::Handlebars;
use serde::Serialize;

#[derive(Debug, Clone)]
pub struct State {
    pub registry: Handlebars<'static>,
}

impl State {
    pub fn new() -> Self {
        let mut state = State {
            registry: Handlebars::new(),
        };
        state.register_template("index.html", "static/index.html");
        state.register_template("login.html", "static/login.html");
        state.register_template("2fa.html", "static/2fa.html");
        state.register_template("settings.html", "static/settings.html");
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

use serde::Serialize;
use tide::Request;
use tide_flash::FlashMessage;

use crate::{prelude::*, registry::State, Claims};

#[derive(Debug)]
pub struct Context<'request, T: Serialize> {
    request: &'request Request<State>,
    flash: Option<Vec<FlashMessage>>,
    claims: Option<Claims>,
    data: Option<T>,
}

impl<'request> Context<'request, ()> {
    pub fn new(request: &'request Request<State>) -> Self {
        Self {
            request,
            flash: request.flash(),
            claims: request.claims(),
            data: None,
        }
    }
}

impl<'request, T: Serialize> Context<'request, T> {
    pub fn with_data(request: &'request Request<State>, data: T) -> Self {
        Self {
            request,
            flash: request.flash(),
            claims: request.claims(),
            data: Some(data),
        }
    }

    pub fn render(self, template: &str) -> Result<String, handlebars::RenderError> {
        self.request.state().render(
            template,
            &serde_json::json!({
                "flash": self.flash,
                "claims": self.claims,
                "data": self.data
            }),
        )
    }
}

use serde::Serialize;
use serde_json::json;
use tide::{Request, StatusCode};

use crate::{prelude::*, registry::State};

pub struct TemplateResponse {}

impl TemplateResponse {
    pub fn new(req: Request<State>, template: &str) -> tide::Result {
        TemplateResponse::with_data(req, template, ())
    }

    pub fn with_data<T: Serialize>(req: Request<State>, template: &str, data: T) -> tide::Result {
        let flash = req.flash();
        let claims = req.claims();
        let template = req.state().render(
            template,
            &json!({
                "flash": flash,
                "claims": claims,
                "data": data
            }),
        );
        let res = tide::Response::builder(StatusCode::Ok)
            .body(template?)
            .content_type(tide::http::mime::HTML)
            .build();
        Ok(res)
    }
}

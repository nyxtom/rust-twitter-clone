use serde::Serialize;
use serde_json::json;
use tide::{http, Request, StatusCode};

use crate::{prelude::*, registry::State};

pub struct TemplateResponse<T: Serialize> {
    request: Request<State>,
    template: String,
    code: StatusCode,
    content_type: http::Mime,
    data: Option<T>,
}

impl TemplateResponse<()> {
    pub fn new(req: Request<State>, template: &str) -> Self {
        TemplateResponse {
            request: req,
            content_type: http::mime::HTML,
            template: String::new(),
            code: StatusCode::Ok,
            data: None,
        }
        .with_template(template)
    }
}

impl<T: Serialize> TemplateResponse<T> {
    pub fn with_data<S: Serialize>(self, data: S) -> TemplateResponse<S> {
        TemplateResponse {
            request: self.request,
            template: self.template,
            content_type: self.content_type,
            code: self.code,
            data: Some(data),
        }
    }

    pub fn with_template(mut self, template: &str) -> Self {
        let (_, ext) = template.rsplit_once('.').unwrap();
        self.content_type = http::Mime::from_extension(ext).unwrap_or(tide::http::mime::HTML);
        self.template = template.into();
        self
    }

    pub fn with_status(mut self, code: StatusCode) -> Self {
        self.code = code;
        self
    }
}

impl<T: Serialize> From<TemplateResponse<T>> for tide::Result {
    fn from(res: TemplateResponse<T>) -> Self {
        let template = res.request.state().render(
            &res.template,
            &json!({
                "flash": res.request.flash(),
                "claims": res.request.claims(),
                "data": res.data
            }),
        )?;

        let res = tide::Response::builder(res.code)
            .body(template)
            .content_type(res.content_type)
            .build();
        Ok(res)
    }
}

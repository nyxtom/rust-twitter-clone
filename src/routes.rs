use serde::{Deserialize, Serialize};
use tide::{Redirect, Request, Server};

use crate::prelude::*;
use crate::registry::State;
use crate::templates::TemplateResponse;

mod account;
mod auth;

#[derive(Serialize, Deserialize)]
pub struct UserForm {
    username: String,
    password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ValidateForm {
    code: String,
}

pub fn configure(app: &mut Server<State>) {
    app.at("/").get(index);
    account::configure(app);
    auth::configure(app);
}

pub async fn index(mut req: Request<State>) -> tide::Result {
    if !req.is_authenticated() {
        TemplateResponse::new(req, "login.html").into()
    } else if req.prevent_totp_redirect() {
        req.logout();
        Ok(Redirect::new("/").into())
    } else if req.requires_totp() {
        TemplateResponse::new(req, "otp.html").into()
    } else {
        TemplateResponse::new(req, "index.html").into()
    }
}

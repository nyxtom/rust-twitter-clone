use serde::{Deserialize, Serialize};
use tide::http::mime;
use tide::{Redirect, Request, Response, Server};
use tide_flash::ext::*;

use crate::prelude::*;
use crate::render_context::Context;
use crate::{registry::State, Claims};

#[derive(Serialize, Deserialize)]
pub struct UserForm {
    username: String,
    password: String,
}

pub fn configure(app: &mut Server<State>) {
    app.at("/").get(index);
    app.at("/login").post(authenticate);
    app.at("/logout").get(logout).post(logout);
}

pub async fn index(req: Request<State>) -> tide::Result {
    let template = if !req.is_authenticated() {
        "login.html"
    } else {
        "index.html"
    };

    let template = Context::new(&req).render(template);
    let res = Response::builder(200)
        .body(template?)
        .content_type(mime::HTML)
        .build();
    Ok(res)
}

pub async fn logout(mut req: Request<State>) -> tide::Result {
    req.logout();
    Ok(Redirect::new("/").into())
}

pub async fn authenticate(mut req: Request<State>) -> tide::Result {
    match req.body_form::<UserForm>().await {
        Ok(form) => {
            if form.username == "foo" && form.password == "bar" {
                let claims = Claims {
                    username: String::from("foo"),
                    exp: 10000000000,
                    sub: String::from("asdf"),
                    uid: 1,
                };
                req.login(claims)?;
                println!("authenticated!");
                Ok(Redirect::new("/").into())
            } else {
                let mut res: tide::Response = Redirect::new("/").into();
                res.flash_error("invalid credentials");
                Ok(res)
            }
        }
        Err(e) => {
            let mut res: tide::Response = Redirect::new("/").into();
            res.flash_error(e.to_string());
            Ok(res)
        }
    }
}

use qrcode::render::svg;
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

#[derive(Serialize, Deserialize)]
pub struct ValidateForm {
    code: String,
}

pub fn configure(app: &mut Server<State>) {
    app.at("/").get(index);
    app.at("/settings").get(settings);
    app.at("/validate-otp").post(validate_otp);
    app.at("/login").post(authenticate);
    app.at("/logout").get(logout).post(logout);
}

pub async fn settings(req: Request<State>) -> tide::Result {
    if !req.is_authenticated() {
        return Ok(Redirect::new("/").into());
    }

    let key_ascii = "12345678901234567890".to_owned();
    let totp = libreauth::oath::TOTPBuilder::new()
        .ascii_key(&key_ascii)
        .finalize()
        .unwrap();

    let uri = totp
        .key_uri_format("TwitterClone", &req.claims().unwrap().username)
        .finalize();
    println!("{}", uri);

    let code = qrcode::QrCode::new(uri).unwrap();
    let image = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#f0f0f0"))
        .build();

    let template =
        Context::with_data(&req, serde_json::json!({ "qrcode": image })).render("settings.html");
    let res = Response::builder(200)
        .body(template?)
        .content_type(mime::HTML)
        .build();
    Ok(res)
}

pub async fn validate_otp(mut req: Request<State>) -> tide::Result {
    if !req.is_authenticated() {
        return Ok(Redirect::new("/").into());
    }

    match req.body_form::<ValidateForm>().await {
        Ok(form) => {
            let key_ascii = "12345678901234567890".to_owned();
            let valid = libreauth::oath::TOTPBuilder::new()
                .ascii_key(&key_ascii)
                .finalize()
                .unwrap()
                .is_valid(&form.code);

            if valid {
                let mut res: Response = Redirect::new("settings").into();
                res.flash_info("valid!");
                Ok(res)
            } else {
                let mut res: tide::Response = Redirect::new("/settings").into();
                res.flash_error("invalid code");
                Ok(res)
            }
        }
        Err(e) => {
            let mut res: tide::Response = Redirect::new("/settings").into();
            res.flash_error(e.to_string());
            Ok(res)
        }
    }
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

use tide::{Redirect, Request, Server};
use uuid::Uuid;
use validator::Validate;

use super::{UserCreateForm, UserForm, ValidateForm};
use crate::prelude::*;
use crate::registry::USERS;
use crate::repos::user::{User, UserStore};
use crate::repos::*;
use crate::templates::TemplateResponse;
use crate::{Claims, State};

pub fn configure(app: &mut Server<State>) {
    app.at("/register").get(register).post(register_post);
    app.at("/login").post(authenticate);
    app.at("/otp").post(authenticate_otp);
}

pub async fn register(req: Request<State>) -> tide::Result {
    TemplateResponse::new(req, "register.html").into()
}

pub async fn register_post(mut req: Request<State>) -> tide::Result {
    match req.body_form::<UserCreateForm>().await {
        Ok(form) => match form.validate() {
            Ok(_) => {
                let res: tide::Response = Redirect::new("/").into();
                match USERS.with_borrow_mut(|db| {
                    db.insert(User {
                        _id: Uuid::new_v4().to_string(),
                        username: form.username,
                        password: form.password,
                        totp_enabled: false,
                        totp_secret: None,
                    })
                }) {
                    Ok(_) => Ok(res),
                    Err(_) => {
                        let mut res: tide::Response = Redirect::new("/register").into();
                        res.flash_error("invalid credentials");
                        Ok(res)
                    }
                }
            }
            Err(e) => {
                let mut res: tide::Response = Redirect::new("/register").into();
                res.flash_error(serde_json::json!(e.field_errors()).to_string());
                Ok(res)
            }
        },
        Err(e) => {
            let mut res: tide::Response = Redirect::new("/register").into();
            res.flash_error(e.to_string());
            Ok(res)
        }
    }
}

pub async fn authenticate(mut req: Request<State>) -> tide::Result {
    match req.body_form::<UserForm>().await {
        Ok(form) => USERS.with_borrow_mut(|db| {
            if let Ok(user) = db.authenticate(form.username, form.password) {
                let claims = Claims {
                    username: user.username.clone(),
                    exp: 10000000000,
                    sub: user.username,
                    uid: user._id,
                    totp_enabled: user.totp_enabled,
                    totp_attempt: 0,
                    totp: None,
                };
                req.login(claims)?;
                Ok(Redirect::new("/").into())
            } else {
                let mut res: tide::Response = Redirect::new("/").into();
                res.flash_error("invalid credentials");
                Ok(res)
            }
        }),
        Err(e) => {
            let mut res: tide::Response = Redirect::new("/").into();
            res.flash_error(e.to_string());
            Ok(res)
        }
    }
}

pub async fn authenticate_otp(mut req: Request<State>) -> tide::Result {
    if !req.is_authenticated() || !req.requires_totp() {
        return Ok(Redirect::new("/").into());
    }

    match req.body_form::<ValidateForm>().await {
        Ok(form) => {
            let key_ascii = req.user().unwrap().totp_secret.clone().unwrap();
            let valid = libreauth::oath::TOTPBuilder::new()
                .ascii_key(&key_ascii)
                .finalize()
                .unwrap()
                .is_valid(&form.code);
            if valid {
                let mut claims = req.claims().unwrap();
                claims.totp_attempt = claims.totp_attempt + 1;
                claims.totp = Some(10000000000);
                req.login(claims)?;
                Ok(Redirect::new("/").into())
            } else {
                let mut claims = req.claims().unwrap();
                claims.totp_attempt = claims.totp_attempt + 1;
                claims.totp = None;
                req.login(claims)?;

                let mut res: tide::Response = Redirect::new("/").into();
                res.flash_error("invalid otp");
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

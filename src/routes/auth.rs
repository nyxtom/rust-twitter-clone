use tide::{Redirect, Request, Server};

use super::UserForm;
use super::ValidateForm;
use crate::prelude::*;
use crate::{Claims, State};

pub fn configure(app: &mut Server<State>) {
    app.at("/login").post(authenticate);
    app.at("/otp").post(authenticate_otp);
}

pub async fn authenticate(mut req: Request<State>) -> tide::Result {
    match req.body_form::<UserForm>().await {
        Ok(form) => {
            // TODO: authenticate username/password with bcrypt (or similar hashing), compare to db
            if form.username == "foo" && form.password == "bar" {
                // TODO: generate proper claims based on authenticated user
                let claims = Claims {
                    username: String::from("foo"),
                    exp: 10000000000,
                    sub: String::from("asdf"),
                    uid: 1,
                    totp_enabled: true,
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
        }
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
            let key_ascii = "12345678901234567890".to_owned();
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

use qrcode::render::svg;
use serde_json::json;
use tide::{Redirect, Request, Response, Server};

use super::ValidateForm;
use crate::prelude::*;
use crate::registry::USERS;
use crate::repos::Store;
use crate::templates::TemplateResponse;
use crate::State;

pub fn configure(app: &mut Server<State>) {
    let state = app.state().clone();
    app.at("/account").authenticated().nest({
        let mut app = tide::with_state(state);
        app.at("/settings").get(settings);
        app.at("/update-2fa").get(update_otp);
        app.at("/validate-otp").post(validate_otp);
        app.at("/logout").get(logout).post(logout);
        app
    });
}

pub async fn logout(mut req: Request<State>) -> tide::Result {
    req.logout();
    Ok(Redirect::new("/").into())
}

pub async fn settings(req: Request<State>) -> tide::Result {
    TemplateResponse::new(req, "settings.html").into()
}

pub async fn update_otp(mut req: Request<State>) -> tide::Result {
    // TODO: turning on 2FA (totp) should require re-authenticating to enable/disable
    // - enabling should display the setup procedure and show the qr code below
    // - qr code should not be visible again after setup (no need to generate totp uri)

    // TODO: obtain shared totp secret based on authenticated user

    let key_ascii = uuid::Uuid::new_v4().to_string();
    req.session_mut().insert("tmp", key_ascii.clone()).unwrap();

    let totp = libreauth::oath::TOTPBuilder::new()
        .ascii_key(&key_ascii)
        .finalize()
        .unwrap();

    let uri = totp
        .key_uri_format("TwitterClone", &req.claims().unwrap().username)
        .finalize();

    let code = qrcode::QrCode::new(uri).unwrap();
    let image = code
        .render()
        .min_dimensions(200, 200)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#f0f0f0"))
        .build();

    TemplateResponse::new(req, "2fa.html")
        .with_data(json!({ "qrcode": image }))
        .into()
}

pub async fn validate_otp(mut req: Request<State>) -> tide::Result {
    match req.body_form::<ValidateForm>().await {
        Ok(form) => {
            // TODO: obtain shared totp secret based on authenticated user
            let key_ascii = req.session().get::<String>("tmp").unwrap();
            let valid = libreauth::oath::TOTPBuilder::new()
                .ascii_key(&key_ascii)
                .finalize()
                .unwrap()
                .is_valid(&form.code);

            if valid {
                USERS.with_borrow_mut(|db| {
                    let uid = req.claims().unwrap().uid;
                    let mut user = req.user().unwrap().clone();
                    user.totp_enabled = true;
                    user.totp_secret = Some(key_ascii.clone());
                    db.update(uid, user).unwrap();
                });
                req.session_mut().remove("tmp");
                let mut res: Response = Redirect::new("/account/settings").into();
                res.flash_info("settings saved!");
                Ok(res)
            } else {
                let mut res: tide::Response = Redirect::new("/account/update-2fa").into();
                res.flash_error("invalid code");
                Ok(res)
            }
        }
        Err(e) => {
            let mut res: tide::Response = Redirect::new("/account/update-2fa").into();
            res.flash_error(e.to_string());
            Ok(res)
        }
    }
}

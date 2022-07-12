use serde::Serialize;
use tide::Request;

use crate::Claims;

pub trait RequestExt {
    fn is_authenticated(&self) -> bool;
    fn requires_totp(&self) -> bool;
    fn clear_totp_redirect(&mut self);
    fn prevent_totp_redirect(&mut self) -> bool;
    fn claims(&self) -> Option<Claims>;
    fn login<Claims: Serialize>(&mut self, claims: Claims) -> Result<(), serde_json::Error>;
    fn logout(&mut self);
}

impl<State> RequestExt for Request<State> {
    fn is_authenticated(&self) -> bool {
        self.session().get::<Claims>("tide.uid").is_some()
    }

    fn requires_totp(&self) -> bool {
        self.claims()
            .map_or(false, |c| c.totp_enabled && c.totp.is_none())
    }

    fn clear_totp_redirect(&mut self) {
        self.session_mut().remove("tide.totp-redirect");
    }

    fn prevent_totp_redirect(&mut self) -> bool {
        if !self.requires_totp() {
            return false;
        }

        if let Some(_) = self.session().get::<i32>("tide.totp-redirect") {
            return true;
        } else {
            self.session_mut().insert("tide.totp-redirect", 1).unwrap();
            return false;
        }
    }

    fn claims(&self) -> Option<Claims> {
        self.session().get::<Claims>("tide.uid")
    }

    fn login<Claims: Serialize>(&mut self, claims: Claims) -> Result<(), serde_json::Error> {
        self.clear_totp_redirect();
        self.session_mut().insert("tide.uid", claims)
    }

    fn logout(&mut self) {
        self.session_mut().destroy();
    }
}

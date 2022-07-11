use serde::Serialize;
use tide::Request;

use crate::Claims;

pub trait RequestExt {
    fn is_authenticated(&self) -> bool;
    fn claims(&self) -> Option<Claims>;
    fn login<Claims: Serialize>(&mut self, claims: Claims) -> Result<(), serde_json::Error>;
    fn logout(&mut self);
}

impl<State> RequestExt for Request<State> {
    fn is_authenticated(&self) -> bool {
        self.session().get::<Claims>("tide.uid").is_some()
    }

    fn claims(&self) -> Option<Claims> {
        self.session().get::<Claims>("tide.uid")
    }

    fn login<Claims: Serialize>(&mut self, claims: Claims) -> Result<(), serde_json::Error> {
        self.session_mut().insert("tide.uid", claims)
    }

    fn logout(&mut self) {
        self.session_mut().destroy();
    }
}

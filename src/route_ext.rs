use async_trait::async_trait;
use tide::{Middleware, Next, Redirect, Request, Route};

use crate::prelude::RequestExt;

pub trait RouteExt {
    fn authenticated(&mut self) -> &mut Self;
}

impl<'a, State: Clone + Send + Sync + 'static> RouteExt for Route<'a, State> {
    fn authenticated(&mut self) -> &mut Self {
        self.with(AuthenticatedMiddleware {});
        self
    }
}

pub struct AuthenticatedMiddleware {}

#[async_trait]
impl<State> Middleware<State> for AuthenticatedMiddleware
where
    State: Clone + Send + Sync + 'static,
{
    async fn handle(&self, mut request: Request<State>, next: Next<'_, State>) -> tide::Result {
        if !request.is_authenticated() {
            // Redirect::new(/login?redirect=url)
            // Redirect::new(/)
            // 401 Unauthorized
            // 403 Forbidden
            return Ok(Redirect::new("/").into());
            //return Ok(Response::new(StatusCode::Unauthorized));
        } else if request.prevent_totp_redirect() {
            // totp should be a one time redirect, require re-login
            request.logout();
            return Ok(Redirect::new("/").into());
        }

        return Ok(next.run(request).await);
    }
}

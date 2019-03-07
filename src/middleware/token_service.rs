use actix_web::{
    error::Result,
    middleware::{Middleware, Response, Started},
    Error as AWError, HttpRequest, HttpResponse,
};

use crate::prelude::*;

pub struct TokenService;

impl<S> Middleware<S> for TokenService {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        unimplemented!()
    }

    fn response(&self, req: &HttpRequest<S>, mut resp: HttpResponse) -> Result<Response> {
        Ok(Response::Done(resp))
    }
}

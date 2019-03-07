use actix_web::{
    Error as AWError,
    error::Result,
    HttpRequest,
    HttpResponse,
    middleware::{
        Middleware,
        Started,
        Response,
    }
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

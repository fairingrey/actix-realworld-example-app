use actix_web::{HttpRequest, HttpResponse, ResponseError};
use futures::Future;

use super::AppState;
use crate::prelude::*;

// Client Messages ↓

#[derive(Debug)]
pub struct GetTags {}

// JSON response objects ↓

#[derive(Serialize)]
pub struct TagsResponse {
    pub tags: Vec<String>,
}

// Route handlers ↓

pub fn get(req: HttpRequest<AppState>) -> impl Future<Item = HttpResponse, Error = Error> {
    req.state()
        .db
        .send(GetTags {})
        .from_err()
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

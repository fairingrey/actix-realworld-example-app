use actix_web::{HttpResponse, web::Data};

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

pub async fn get(state: Data<AppState>) -> Result<HttpResponse, Error> { 
    let res = state
        .db
        .send(GetTags {}).await??;

    Ok(HttpResponse::Ok().json(res))
}

use actix_web::{HttpRequest, HttpResponse, Json, Path, ResponseError};
use futures::{future::result, Future};

use super::AppState;
use crate::models::User;
use crate::prelude::*;
use crate::utils::auth::{authenticate, Auth};

// Extractors ↓

#[derive(Debug, Deserialize)]
pub struct ProfilePath {
    username: String,
}

// Client Messages ↓

#[derive(Debug)]
pub struct GetProfile {
    pub auth: Auth,
    pub username: String,
}

// JSON response objects ↓

#[derive(Debug, Serialize)]
pub struct ProfileResponse {
    pub profile: ProfileResponseInner,
}

#[derive(Debug, Serialize)]
pub struct ProfileResponseInner {
    username: String,
    bio: Option<String>,
    image: Option<String>,
    following: bool,
}

// Route handlers ↓

//pub fn get((path, req): (Path<ProfilePath>, HttpRequest<AppState>)) -> impl Future<Item = HttpResponse, Error = Error> {
//
//    let db = req.state().db.clone();
//
//    authenticate(&req)
//        .and_then(move |auth| {
//            db.send(GetProfile {
//                auth,
//                username: path.username,
//            }).from_err()
//        })
//        .and_then(|res| match res {
//
//        })
//}

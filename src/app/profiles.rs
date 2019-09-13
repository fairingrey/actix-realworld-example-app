use actix_web::{HttpRequest, HttpResponse, web::Path, web::Data};
use actix_http::error::ResponseError;
use futures::Future;

use super::AppState;
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
    // auth is option in case authentication fails or isn't present
    pub auth: Option<Auth>,
    pub username: String,
}

#[derive(Debug)]
pub struct FollowProfile {
    pub auth: Auth,
    pub username: String,
}

#[derive(Debug)]
pub struct UnfollowProfile {
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
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}

// Route handlers ↓

pub fn get(
    state: Data<AppState>,
    (path, req): (Path<ProfilePath>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .then(move |auth| {
            db.send(GetProfile {
                auth: auth.ok(),
                username: path.username.to_owned(),
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn follow(
    state: Data<AppState>,
    (path, req): (Path<ProfilePath>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .and_then(move |auth| {
            db.send(FollowProfile {
                auth,
                username: path.username.to_owned(),
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn unfollow(
    state: Data<AppState>,
    (path, req): (Path<ProfilePath>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .and_then(move |auth| {
            db.send(UnfollowProfile {
                auth,
                username: path.username.to_owned(),
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

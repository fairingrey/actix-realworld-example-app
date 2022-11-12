use actix_web::{web::Data, web::Path, HttpRequest, HttpResponse};

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

pub async fn get(
    state: Data<AppState>,
    (path, req): (Path<ProfilePath>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req)
        .await
        .map(|auth| Some(auth))
        .unwrap_or(None);

    let res = state
        .db
        .send(GetProfile {
            auth,
            username: path.username.to_owned(),
        })
        .await??;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn follow(
    state: Data<AppState>,
    (path, req): (Path<ProfilePath>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    let res = state
        .db
        .send(FollowProfile {
            auth,
            username: path.username.to_owned(),
        })
        .await??;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn unfollow(
    state: Data<AppState>,
    (path, req): (Path<ProfilePath>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    let res = state
        .db
        .send(UnfollowProfile {
            auth,
            username: path.username.to_owned(),
        })
        .await??;

    Ok(HttpResponse::Ok().json(res))
}

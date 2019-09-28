use actix_web::{HttpRequest, HttpResponse, web::Json, web::Path, web::Data};
use actix_http::error::ResponseError;
use futures::{future::result, Future};
use validator::Validate;

use super::super::AppState;
use crate::app::profiles::ProfileResponseInner;
use crate::prelude::*;
use crate::utils::{
    auth::{authenticate, Auth},
    CustomDateTime,
};

#[derive(Debug, Deserialize)]
pub struct In<T> {
    comment: T,
}

// Extractors ↓

use super::ArticlePath;

#[derive(Debug, Deserialize)]
pub struct ArticleCommentPath {
    slug: String,
    comment_id: i32,
}

// Client Messages ↓

#[derive(Debug, Validate, Deserialize)]
pub struct AddComment {
    #[validate(length(min = "1", message = "fails validation - cannot be empty"))]
    pub body: String,
}

#[derive(Debug)]
pub struct AddCommentOuter {
    pub auth: Auth,
    pub slug: String,
    pub comment: AddComment,
}

#[derive(Debug)]
pub struct GetComments {
    pub auth: Option<Auth>,
    pub slug: String,
}

#[derive(Debug)]
pub struct DeleteComment {
    pub auth: Auth,
    pub slug: String,
    pub comment_id: i32,
}

// JSON response objects ↓

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub comment: CommentResponseInner,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentResponseInner {
    pub id: i32,
    pub created_at: CustomDateTime,
    pub updated_at: CustomDateTime,
    pub body: String,
    pub author: ProfileResponseInner,
}

#[derive(Debug, Serialize)]
pub struct CommentListResponse {
    pub comments: Vec<CommentResponseInner>,
}

// Route handlers ↓

pub fn add(
    state: Data<AppState>,
    (path, form, req): (
        Path<ArticlePath>,
        Json<In<AddComment>>,
        HttpRequest,
    ),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let comment = form.into_inner().comment;

    let db = state.db.clone();

    result(comment.validate())
        .from_err()
        .and_then(move |_| authenticate(&state, &req))
        .and_then(move |auth| {
            db.send(AddCommentOuter {
                auth,
                slug: path.slug.to_owned(),
                comment,
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn list(
    state: Data<AppState>,
    (path, req): (Path<ArticlePath>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .then(move |auth| {
            db.send(GetComments {
                auth: auth.ok(),
                slug: path.slug.to_owned(),
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn delete(
    state: Data<AppState>,
    (path, req): (Path<ArticleCommentPath>, HttpRequest),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = state.db.clone();

    authenticate(&state, &req)
        .and_then(move |auth| {
            db.send(DeleteComment {
                auth,
                slug: path.slug.to_owned(),
                comment_id: path.comment_id.to_owned(),
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().finish()),
            Err(e) => Ok(e.error_response()),
        })
}

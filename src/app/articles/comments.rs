use actix_web::{HttpRequest, HttpResponse, Json, Path, ResponseError};
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
    comment_id: usize,
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
    pub comment_id: usize,
}

// JSON response objects ↓

#[derive(Debug, Serialize)]
pub struct CommentResponse {
    pub comment: CommentResponseInner,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentResponseInner {
    pub id: usize,
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
    (path, form, req): (
        Path<ArticlePath>,
        Json<In<AddComment>>,
        HttpRequest<AppState>,
    ),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let comment = form.into_inner().comment;

    let db = req.state().db.clone();

    result(comment.validate())
        .from_err()
        .and_then(move |_| authenticate(&req))
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
    (path, req): (Path<ArticlePath>, HttpRequest<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = req.state().db.clone();

    authenticate(&req)
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
    (path, req): (Path<ArticleCommentPath>, HttpRequest<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = req.state().db.clone();

    authenticate(&req)
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

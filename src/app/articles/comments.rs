use actix_web::{web::Data, web::Json, web::Path, HttpRequest, HttpResponse};
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
    #[validate(length(min = 1, message = "fails validation - cannot be empty"))]
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

pub async fn add(
    state: Data<AppState>,
    (path, form, req): (Path<ArticlePath>, Json<In<AddComment>>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let comment = form.into_inner().comment;
    comment.validate()?;

    let auth = authenticate(&state, &req).await?;
    let res = state
        .db
        .send(AddCommentOuter {
            auth,
            slug: path.slug.to_owned(),
            comment,
        })
        .await??;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn list(
    state: Data<AppState>,
    (path, req): (Path<ArticlePath>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req)
        .await
        .map(|auth| Some(auth))
        .unwrap_or(None);

    let res = state
        .db
        .send(GetComments {
            auth,
            slug: path.slug.to_owned(),
        })
        .await??;

    Ok(HttpResponse::Ok().json(res))
}

pub async fn delete(
    state: Data<AppState>,
    (path, req): (Path<ArticleCommentPath>, HttpRequest),
) -> Result<HttpResponse, Error> {
    let auth = authenticate(&state, &req).await?;
    let res = state
        .db
        .send(DeleteComment {
            auth,
            slug: path.slug.to_owned(),
            comment_id: path.comment_id.to_owned(),
        })
        .await??;

    Ok(HttpResponse::Ok().json(res))
}

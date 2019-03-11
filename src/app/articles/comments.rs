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
pub struct CreateComment {
    #[validate(length(min = "1"))]
    pub body: String,
}

#[derive(Debug)]
pub struct CreateCommentOuter {
    pub auth: Auth,
    pub slug: String,
    pub comment: CreateComment,
}

#[derive(Debug)]
pub struct GetComments {
    pub auth: Option<Auth>,
    pub slug: String,
}

#[derive(Debug)]
pub struct DeleteComment {
    pub auth: Auth,
    pub path: ArticleCommentPath,
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

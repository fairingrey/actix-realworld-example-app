pub mod comments;

use actix_web::{HttpRequest, HttpResponse, Json, ResponseError};
use futures::{future::result, Future};
use validator::Validate;

use super::AppState;
use crate::app::profiles::ProfileResponseInner;
use crate::models::Article;
use crate::prelude::*;
use crate::utils::{
    auth::{authenticate, Auth},
    CustomDateTime,
};

#[derive(Debug, Deserialize)]
pub struct In<T> {
    article: T,
}

// Extractors ↓

// Client Messages ↓

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateArticle {
    title: String,
    description: String,
    body: String,
    tag_list: Vec<String>,
}

#[derive(Debug)]
pub struct CreateArticleOuter {
    auth: Auth,
    new_article: CreateArticle,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleChange {
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub tag_list: Option<Vec<String>>,
}

// JSON response objects ↓

#[derive(Debug, Serialize)]
pub struct ArticleResponse {
    pub article: ArticleResponseInner,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleResponseInner {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
    pub created_at: CustomDateTime,
    pub updated_at: CustomDateTime,
    pub favorited: bool,
    pub favorites_count: i64,
    pub author: ProfileResponseInner,
}

// Route handlers ↓

//pub fn create(
//    (form, req): (Json<In<CreateArticle>>, HttpRequest<AppState>)
//) -> impl Future<Item = HttpResponse, Error = Error> {
//    let db = req.state().db.clone();
//
//    authenticate(&req)
//        .and_then(move |auth| {
//            db.send(CreateArticleOuter {
//                auth,
//                new_article: form.into_inner().article,
//            }).from_err()
//        })
//        .and_then(|res| match res {
//            Ok(res) => Ok(HttpResponse::Ok().json(res)),
//            Err(e) => Ok(e.error_response()),
//        })
//}

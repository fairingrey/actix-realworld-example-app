pub mod comments;

use actix_web::{HttpRequest, HttpResponse, Json, Path, Query, ResponseError};
use futures::{future::result, Future};
use validator::Validate;

use super::AppState;
use crate::app::profiles::ProfileResponseInner;
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

#[derive(Debug, Deserialize)]
pub struct ArticlePath {
    slug: String,
}

#[derive(Debug, Deserialize)]
pub struct ArticlesParams {
    tag: Option<String>,
    author: Option<String>,
    favorited: Option<String>,
    limit: Option<usize>,
    offset: Option<usize>,
}

#[derive(Debug, Deserialize)]
pub struct FeedParams {
    limit: Option<usize>,
    offset: Option<usize>,
}

// Client Messages ↓

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateArticle {
    #[validate(length(min = "1"))]
    pub title: String,
    pub description: String,
    pub body: String,
    pub tag_list: Vec<String>,
}

#[derive(Debug)]
pub struct CreateArticleOuter {
    pub auth: Auth,
    pub article: CreateArticle,
}

#[derive(Debug)]
pub struct GetArticle {
    pub auth: Option<Auth>,
    pub slug: String,
}

#[derive(Debug, Validate, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateArticle {
    #[validate(length(min = "1"))]
    pub title: Option<String>,
    pub description: Option<String>,
    pub body: Option<String>,
    pub tag_list: Option<Vec<String>>,
}

#[derive(Debug)]
pub struct UpdateArticleOuter {
    pub auth: Auth,
    pub article: UpdateArticle,
}

#[derive(Debug)]
pub struct DeleteArticle {
    pub auth: Auth,
    pub slug: String,
}

#[derive(Debug)]
pub struct GetArticles {
    pub auth: Option<Auth>,
    pub params: ArticlesParams,
}

#[derive(Debug)]
pub struct GetFeed {
    pub auth: Auth,
    pub params: FeedParams,
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
    pub favorites_count: usize,
    pub author: ProfileResponseInner,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleListResponse {
    pub articles: Vec<ArticleResponseInner>,
    pub articles_count: usize,
}

// Route handlers ↓

pub fn create(
    (form, req): (Json<In<CreateArticle>>, HttpRequest<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let article = form.into_inner().article;
    let db = req.state().db.clone();

    result(article.validate())
        .from_err()
        .and_then(move |_| authenticate(&req))
        .and_then(move |auth| db.send(CreateArticleOuter { auth, article }).from_err())
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn get(
    (path, req): (Path<ArticlePath>, HttpRequest<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = req.state().db.clone();

    authenticate(&req)
        .then(move |auth| {
            db.send(GetArticle {
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

pub fn update(
    (form, req): (Json<In<UpdateArticle>>, HttpRequest<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let article = form.into_inner().article;

    let db = req.state().db.clone();

    result(article.validate())
        .from_err()
        .and_then(move |_| authenticate(&req))
        .and_then(move |auth| db.send(UpdateArticleOuter { auth, article }).from_err())
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn delete(
    (path, req): (Path<ArticlePath>, HttpRequest<AppState>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    authenticate(&req)
        .and_then(move |auth| {
            req.state()
                .db
                .send(DeleteArticle {
                    auth,
                    slug: path.slug.to_owned(),
                })
                .from_err()
        })
        .and_then(|res| match res {
            Ok(_) => Ok(HttpResponse::Ok().json(())),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn list(
    (req, params): (HttpRequest<AppState>, Query<ArticlesParams>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = req.state().db.clone();

    authenticate(&req)
        .then(move |auth| {
            db.send(GetArticles {
                auth: auth.ok(),
                params: params.into_inner(),
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

pub fn feed(
    (req, params): (HttpRequest<AppState>, Query<FeedParams>),
) -> impl Future<Item = HttpResponse, Error = Error> {
    let db = req.state().db.clone();

    authenticate(&req)
        .and_then(move |auth| {
            db.send(GetFeed {
                auth,
                params: params.into_inner(),
            })
            .from_err()
        })
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
}

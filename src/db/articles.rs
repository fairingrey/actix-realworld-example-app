use actix::prelude::*;
use diesel::prelude::*;

use super::DbExecutor;
use crate::app::articles::{UpdateArticle, CreateArticleOuter};
use crate::models::{Article, ArticleChange, NewArticle, NewFavoriteArticle};

// handler implementations ↓


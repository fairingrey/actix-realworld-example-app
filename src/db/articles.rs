use actix::prelude::*;
use diesel::prelude::*;
use slug::slugify;

use super::DbExecutor;
use crate::app::articles::{
    ArticleResponse, ArticleResponseInner, CreateArticleOuter, UpdateArticle,
};
use crate::app::profiles::ProfileResponseInner;
use crate::models::{
    Article, ArticleChange, ArticleTag, NewArticle, NewArticleTag, NewFavoriteArticle,
};
use crate::prelude::*;
use crate::utils::custom_type::CustomDateTime;

// handler implementations ↓

impl Message for CreateArticleOuter {
    type Result = Result<ArticleResponse>;
}

impl Handler<CreateArticleOuter> for DbExecutor {
    type Result = Result<ArticleResponse>;

    fn handle(&mut self, msg: CreateArticleOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::articles::dsl::*;

        let conn = &self.0.get().expect("Connection couldn't be opened");

        let author = msg.auth.user;

        let new_article = NewArticle {
            author_id: author.id,
            slug: slugify(&msg.article.title),
            title: msg.article.title,
            description: msg.article.description,
            body: msg.article.body,
        };
        let article = diesel::insert_into(articles)
            .values(&new_article)
            .get_result::<Article>(conn)?;

        let tag_list = msg.article.tag_list;

        let _ = tag_list.iter().map(|tag| {
            use crate::schema::article_tags;

            diesel::insert_into(article_tags::table)
                .values(NewArticleTag {
                    article_id: article.id,
                    tag_name: tag.to_owned(),
                })
                .get_result::<ArticleTag>(conn)
        });

        Ok(ArticleResponse {
            article: ArticleResponseInner {
                slug: article.slug,
                title: article.title,
                description: article.description,
                body: article.body,
                tag_list,
                created_at: CustomDateTime(article.created_at),
                updated_at: CustomDateTime(article.updated_at),
                favorited: false,
                favorites_count: 0,
                author: ProfileResponseInner {
                    username: author.username,
                    bio: author.bio,
                    image: author.image,
                    following: false, // <- note you can't follow yourself
                },
            },
        })
    }
}

impl Message for UpdateArticle {
    type Result = Result<ArticleResponse>;
}

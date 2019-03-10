use actix::prelude::*;
use blob_uuid::to_blob;
use diesel::prelude::*;
use slug::slugify;
use uuid::Uuid;

use super::{DbExecutor, PooledConn};
use crate::app::articles::{
    ArticleListResponse, ArticleResponse, ArticleResponseInner, CreateArticleOuter, GetArticle, GetArticles, GetFeed,
    UpdateArticleOuter, DeleteArticle,
};
use crate::app::profiles::ProfileResponseInner;
use crate::models::{
    Article, ArticleChange, ArticleTag, NewArticle, NewArticleTag, NewFavoriteArticle,
};
use crate::prelude::*;
use crate::utils::custom_type::CustomDateTime;

// message handler implementations ↓

impl Message for CreateArticleOuter {
    type Result = Result<ArticleResponse>;
}

impl Handler<CreateArticleOuter> for DbExecutor {
    type Result = Result<ArticleResponse>;

    fn handle(&mut self, msg: CreateArticleOuter, _: &mut Self::Context) -> Self::Result {
        use crate::schema::articles;

        let conn = &self.0.get().expect("Connection couldn't be opened");

        let author = msg.auth.user;

        // Generating the Uuid here since it will help make a unique slug
        // This is for when some articles may have similar titles such that they generate the same slug
        let new_article_id = Uuid::new_v4();
        let slug = format!(
            "{}-{}",
            to_blob(&new_article_id),
            slugify(&msg.article.title)
        );

        let new_article = NewArticle {
            id: new_article_id,
            author_id: author.id,
            slug,
            title: msg.article.title,
            description: msg.article.description,
            body: msg.article.body,
        };
        let article = diesel::insert_into(articles::table)
            .values(&new_article)
            .get_result::<Article>(conn)?;

        let tag_list = msg.article.tag_list;

        for tag in tag_list.iter() {
            add_tag(article.id, tag, conn)?;
        }

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

impl Message for GetArticle {
    type Result = Result<ArticleResponse>;
}

impl Handler<GetArticle> for DbExecutor {
    type Result = Result<ArticleResponse>;

    fn handle(&mut self, msg: GetArticle, _: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}

impl Message for GetFeed {
    type Result = Result<ArticleListResponse>;
}

impl Handler<GetFeed> for DbExecutor {
    type Result = Result<ArticleListResponse>;

    fn handle(&mut self, msg: GetFeed, _: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}

impl Message for UpdateArticleOuter {
    type Result = Result<ArticleResponse>;
}

impl Handler<UpdateArticleOuter> for DbExecutor {
    type Result = Result<ArticleResponse>;

    fn handle(&mut self, msg: UpdateArticleOuter, _: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}

impl Message for DeleteArticle {
    type Result = Result<ArticleResponse>;
}

impl Handler<DeleteArticle> for DbExecutor {
    type Result = Result<ArticleResponse>;

    fn handle(&mut self, msg: DeleteArticle, _: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}

impl Message for GetArticles {
    type Result = Result<ArticleListResponse>;
}

impl Handler<GetArticles> for DbExecutor {
    type Result = Result<ArticleListResponse>;

    fn handle(&mut self, msg: GetArticles, _: &mut Self::Context) -> Self::Result {
        unimplemented!()
    }
}

fn add_tag(article_id: Uuid, tag_name: &str, conn: &PooledConn) -> Result<ArticleTag> {
    use crate::schema::article_tags;

    diesel::insert_into(article_tags::table)
        .values(NewArticleTag {
            article_id,
            tag_name: tag_name.to_owned(),
        })
        .get_result::<ArticleTag>(conn)
        .map_err(std::convert::Into::into) // <- clippy doesn't like it when I write this as |e| e.into() so...
}

table! {
    articles (id) {
        id -> Uuid,
        author_id -> Uuid,
        slug -> Text,
        title -> Text,
        description -> Text,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    article_tags (article_id, tag_name) {
        article_id -> Uuid,
        tag_name -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    comments (id) {
        id -> Int4,
        article_id -> Uuid,
        user_id -> Uuid,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    favorite_articles (user_id, article_id) {
        user_id -> Uuid,
        article_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    followers (user_id, follower_id) {
        user_id -> Uuid,
        follower_id -> Uuid,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Text,
        email -> Varchar,
        password -> Text,
        bio -> Nullable<Text>,
        image -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(article_tags -> articles (article_id));
joinable!(articles -> users (author_id));
joinable!(comments -> articles (article_id));
joinable!(comments -> users (user_id));
joinable!(favorite_articles -> articles (article_id));
joinable!(favorite_articles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    articles,
    article_tags,
    comments,
    favorite_articles,
    followers,
    users,
);

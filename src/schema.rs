table! {
    articles (id) {
        id -> Int4,
        author_id -> Int4,
        slug -> Varchar,
        title -> Varchar,
        description -> Varchar,
        body -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    article_tags (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        article_id -> Int4,
        tag_name -> Varchar,
    }
}

table! {
    comments (id) {
        id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        article_id -> Int4,
        user_id -> Int4,
        body -> Varchar,
    }
}

table! {
    credentials (id) {
        id -> Int4,
        user_id -> Int4,
        password_hash -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    favorite_articles (id) {
        id -> Int4,
        user_id -> Int4,
        article_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    followers (id) {
        id -> Int4,
        user_id -> Int4,
        follower_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        email -> Varchar,
        bio -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(article_tags -> articles (article_id));
joinable!(articles -> users (author_id));
joinable!(comments -> articles (article_id));
joinable!(comments -> users (user_id));
joinable!(credentials -> users (user_id));
joinable!(favorite_articles -> articles (article_id));
joinable!(favorite_articles -> users (user_id));

allow_tables_to_appear_in_same_query!(
    articles,
    article_tags,
    comments,
    credentials,
    favorite_articles,
    followers,
    users,
);

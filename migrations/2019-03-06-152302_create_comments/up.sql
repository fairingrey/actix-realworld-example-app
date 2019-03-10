CREATE TABLE comments (
    id SERIAL PRIMARY KEY,
    article_id UUID NOT NULL REFERENCES articles (id),
    user_id UUID NOT NULL REFERENCES users (id),
    body TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX comments_article_id_idx ON comments (article_id);
CREATE INDEX comments_user_id_idx ON comments (user_id);

SELECT diesel_manage_updated_at('comments');

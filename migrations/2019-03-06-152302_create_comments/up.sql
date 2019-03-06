-- Your SQL goes here
CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  article_id UUID NOT NULL REFERENCES ARTICLES (ID),
  user_id UUID NOT NULL REFERENCES USERS (ID),
  body TEXT NOT NULL
);

SELECT diesel_manage_updated_at('comments');

CREATE INDEX comments_article_id ON comments (article_id);
CREATE INDEX comments_user_id ON comments (user_id);

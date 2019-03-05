CREATE TABLE comments (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  article_id INT NOT NULL REFERENCES ARTICLES (ID),
  user_id INT NOT NULL REFERENCES USERS (ID),
  body VARCHAR NOT NULL
);

CREATE INDEX comments_article_id ON comments (article_id);
CREATE INDEX comments_user_id ON comments (user_id);

CREATE TABLE articles (
  id SERIAL PRIMARY KEY,
  author_id INT NOT NULL REFERENCES users (ID),
  slug VARCHAR(300)  NOT NULL,
  title VARCHAR(200) NOT NULL,
  description VARCHAR(500) NOT NULL,
  body VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (author_id, slug)
);

CREATE TABLE favorite_articles (
  id SERIAL PRIMARY KEY,
  user_id int NOT NULL REFERENCES users (ID),
  article_id int NOT NULL REFERENCES articles (ID),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (user_id, article_id)
);

CREATE INDEX favorite_articles_user_id ON favorite_articles (user_id);
CREATE INDEX favorite_articles_article_id ON favorite_articles (article_id);

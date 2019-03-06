-- Your SQL goes here
CREATE TABLE articles (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  author_id UUID NOT NULL REFERENCES users (id),
  slug TEXT NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  body TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  UNIQUE (author_id, slug)
);

SELECT diesel_manage_updated_at('articles');

CREATE TABLE favorite_articles (
  user_id UUID UNIQUE NOT NULL REFERENCES users (ID),
  article_id UUID UNIQUE NOT NULL REFERENCES articles (ID),
  PRIMARY KEY (user_id, article_id),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('favorite_articles');

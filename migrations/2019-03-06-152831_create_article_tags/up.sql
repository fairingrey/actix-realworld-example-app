-- Your SQL goes here

CREATE TABLE article_tags (
  article_id UUID UNIQUE NOT NULL REFERENCES articles (id),
  tag_name TEXT UNIQUE NOT NULL,
  PRIMARY KEY (article_id, tag_name),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('article_tags');

-- The article slug is used as a part of URL like /api/articles/:slug so
-- it must be unique among all articles.
ALTER TABLE articles DROP CONSTRAINT articles_author_id_slug_key;
ALTER TABLE articles ADD CONSTRAINT articles_slug_key UNIQUE (slug);

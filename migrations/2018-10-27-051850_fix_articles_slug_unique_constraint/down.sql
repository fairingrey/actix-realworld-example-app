ALTER TABLE articles DROP CONSTRAINT articles_slug_key;
ALTER TABLE articles ADD CONSTRAINT articles_author_id_slug_key UNIQUE (author_id, slug);

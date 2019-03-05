-- Experimentally we associates tag names with articles directly,
-- instead of defining the tags table.
-- Though this is not a good idea in general, it can meet
-- this demo app's specification more easily.
--
-- Props:
--   - It makes simple to update article tags.
-- Cons:
--   - There is no way to add meta data to tags.
--   - To rename a tag, we need to update all records selected by its name.

CREATE TABLE article_tags (
  id SERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  article_id INT NOT NULL REFERENCES articles (id),
  tag_name VARCHAR NOT NULL,
  UNIQUE(article_id, tag_name)
);

CREATE INDEX article_tags_article_id ON article_tags (article_id);
CREATE INDEX article_tags_tag_name ON article_tags (tag_name);

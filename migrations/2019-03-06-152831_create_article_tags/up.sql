CREATE TABLE article_tags (
    article_id UUID NOT NULL REFERENCES articles (id),
    tag_name TEXT NOT NULL,
    PRIMARY KEY (article_id, tag_name),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX article_tags_article_id_idx ON article_tags (article_id);
CREATE INDEX article_tags_tag_name_idx ON article_tags (tag_name);

SELECT diesel_manage_updated_at('article_tags');

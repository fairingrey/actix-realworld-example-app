CREATE TABLE followers (
    user_id UUID NOT NULL REFERENCES users (id),
    follower_id UUID NOT NULL REFERENCES users (id),
    PRIMARY KEY (user_id, follower_id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

-- followers_user_id_idx index may already be created, as postgres automatically indexes only the leftmost column in a composite key
-- see https://stackoverflow.com/questions/11352056/postgresql-composite-primary-key
CREATE INDEX followers_user_id_idx ON followers (user_id);
CREATE INDEX followers_follower_user_id_idx ON followers (follower_id);

-- also prevent users from following themselves
-- this should be caught in application logic and return a 422, but if it doesn't...
ALTER TABLE followers ADD CONSTRAINT user_id_cannot_be_equal_to_follower_id_chk CHECK (user_id != follower_id);

SELECT diesel_manage_updated_at('followers');

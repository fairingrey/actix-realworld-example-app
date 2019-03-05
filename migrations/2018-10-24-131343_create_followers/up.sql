CREATE TABLE followers (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL REFERENCES users (id),
  follower_id INT NOT NULL REFERENCES users (id),
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  UNIQUE (user_id, follower_id)
);

CREATE INDEX followers_user_id ON followers (user_id);
CREATE INDEX followers_follower_id ON followers (follower_id);

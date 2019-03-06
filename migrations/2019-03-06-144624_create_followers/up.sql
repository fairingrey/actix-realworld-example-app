-- Your SQL goes here
CREATE TABLE followers (
  user_id UUID UNIQUE NOT NULL REFERENCES users (id),
  follower_id UUID UNIQUE NOT NULL REFERENCES users (id),
  PRIMARY KEY (user_id, follower_id),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

SELECT diesel_manage_updated_at('followers');

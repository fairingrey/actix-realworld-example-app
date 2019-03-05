CREATE TABLE users (
  id SERIAL PRIMARY KEY,
  username VARCHAR(20) NOT NULL,
  email VARCHAR(512) NOT NULL,
  bio VARCHAR,
  image VARCHAR,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX ON users (username);

-- Separate credential data from the users table.
-- We should fetch such data only if necessary.
CREATE TABLE credentials (
  id SERIAL PRIMARY KEY,
  user_id INT NOT NULL REFERENCES users (id),
  password_hash VARCHAR NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

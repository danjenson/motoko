CREATE TABLE users (
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  display_name TEXT NOT NULL,
  name TEXT UNIQUE NOT NULL,
  email TEXT UNIQUE NOT NULL
);
CREATE INDEX users_created_at_idx ON users(created_at);
CREATE INDEX users_updated_at_idx ON users(updated_at);

SELECT manage_updated_at('users');

-- TEXT with CHECK advised over VARCHAR(n): https://tinyurl.com/ychbl4xw
ALTER TABLE users
  ADD CONSTRAINT display_name_max_length
  CHECK (length(display_name) < 100);

ALTER TABLE users
  ADD CONSTRAINT name_max_length
  CHECK (length(display_name) < 20);

ALTER TABLE users
  ADD CONSTRAINT email_max_length
  CHECK (length(display_name) < 100);

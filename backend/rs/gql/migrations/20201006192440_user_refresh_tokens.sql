CREATE TABLE user_refresh_tokens (
  expires_at TIMESTAMPTZ NOT NULL,
  user_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
  value TEXT PRIMARY KEY NOT NULL
);
CREATE INDEX user_refresh_tokens_expires_at_idx
  ON user_refresh_tokens(expires_at);

CREATE TABLE models (
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  dataview_uuid UUID NOT NULL REFERENCES dataviews(uuid) ON DELETE CASCADE,
  uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name TEXT NOT NULL,
  target TEXT,
  features TEXT[] NOT NULL,
  args JSON,
  status STATUS DEFAULT 'queued' NOT NULL,
  decisions JSON,
  evaluation JSON,
  error JSON
);
CREATE INDEX models_created_at_idx ON models(created_at);
CREATE INDEX models_updated_at_idx ON models(updated_at);

SELECT manage_updated_at('models');

ALTER TABLE models
  ADD CONSTRAINT name_max_length
  CHECK (length(name) < 100);

ALTER TABLE models
  ADD CONSTRAINT at_least_one_feature
  CHECK (cardinality(features) > 0);

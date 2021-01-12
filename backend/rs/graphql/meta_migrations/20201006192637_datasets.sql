CREATE TABLE datasets (
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  project_uuid UUID NOT NULL REFERENCES projects(uuid) ON DELETE CASCADE,
  uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name TEXT NOT NULL,
  uri TEXT NOT NULL,
  status STATUS DEFAULT 'queued' NOT NULL,
  error JSON
);
CREATE INDEX datasets_created_at_idx ON datasets(created_at);
CREATE INDEX datasets_updated_at_idx ON datasets(updated_at);

SELECT manage_updated_at('datasets');

ALTER TABLE datasets
  ADD CONSTRAINT name_max_length
  CHECK (length(name) < 100);

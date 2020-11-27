CREATE TABLE analyses (
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  dataset_uuid UUID NOT NULL REFERENCES datasets(uuid) ON DELETE CASCADE,
  dataview_uuid UUID NOT NULL REFERENCES dataviews(uuid),
  uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name TEXT NOT NULL
);
CREATE INDEX analyses_created_at_idx ON analyses(created_at);
CREATE INDEX analyses_updated_at_idx ON analyses(updated_at);

SELECT manage_updated_at('analyses');

ALTER TABLE analyses
  ADD CONSTRAINT name_max_length
  CHECK (length(name) < 100);

ALTER TABLE dataviews
  ADD CONSTRAINT dataviews_analyses_uuid_fkey
  FOREIGN KEY (analysis_uuid) REFERENCES analyses(uuid) ON DELETE CASCADE;

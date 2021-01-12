CREATE TYPE DATAVIEW_OPERATION AS ENUM(
  'create',
  'filter',
  'mutate',
  'select',
  'sort',
  'summarize'
);

CREATE TABLE dataviews (
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  analysis_uuid UUID NOT NULL,
  uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  parent_uuid UUID NOT NULL REFERENCES dataviews(uuid) ON DELETE CASCADE,
  operation DATAVIEW_OPERATION DEFAULT 'create' NOT NULL,
  args JSON,
  status STATUS DEFAULT 'queued' NOT NULL,
  error JSON
);
CREATE INDEX dataviews_created_at_idx ON dataviews(created_at);
CREATE INDEX dataviews_updated_at_idx ON dataviews(updated_at);

SELECT manage_updated_at('dataviews');

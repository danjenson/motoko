CREATE TYPE STATISTIC_NAME AS ENUM(
  'correlation',
  'mean',
  'median',
  'mode'
);

CREATE TABLE statistics (
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  dataview_uuid UUID NOT NULL REFERENCES dataviews(uuid) ON DELETE CASCADE,
  uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name STATISTIC_NAME NOT NULL,
  args JSON NOT NULL,
  status STATUS DEFAULT 'queued' NOT NULL,
  value DOUBLE PRECISION
);
CREATE INDEX statistics_created_at_idx ON statistics(created_at);
CREATE INDEX statistics_updated_at_idx ON statistics(updated_at);

SELECT manage_updated_at('statistics');

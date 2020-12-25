CREATE TYPE PLOT_TYPE AS ENUM(
  'bar',
  'histogram',
  'line',
  'scatter',
  'smooth'
);

CREATE TABLE plots (
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  dataview_uuid UUID NOT NULL REFERENCES dataviews(uuid) ON DELETE CASCADE,
  uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name TEXT NOT NULL,
  type PLOT_TYPE NOT NULL,
  args JSON NOT NULL,
  status STATUS DEFAULT 'queued' NOT NULL
);
CREATE INDEX plots_created_at_idx ON plots(created_at);
CREATE INDEX plots_updated_at_idx ON plots(updated_at);

SELECT manage_updated_at('plots');

ALTER TABLE plots
  ADD CONSTRAINT name_max_length
  CHECK (length(name) < 100);

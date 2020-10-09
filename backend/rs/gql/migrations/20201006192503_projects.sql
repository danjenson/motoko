CREATE TABLE projects (
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  uuid UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name TEXT NOT NULL,
  is_public BOOLEAN NOT NULL DEFAULT false
);
CREATE INDEX projects_created_at_idx ON projects(created_at);
CREATE INDEX projects_updated_at_idx ON projects(updated_at);

SELECT manage_updated_at('projects');

ALTER TABLE projects
  ADD CONSTRAINT name_max_length
  CHECK (length(name) < 100);

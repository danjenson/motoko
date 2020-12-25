CREATE TYPE PROJECT_USER_ROLE AS ENUM(
  'viewer',
  'editor',
  'admin'
);

CREATE TABLE project_user_roles (
  created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  project_uuid UUID NOT NULL REFERENCES projects(uuid) ON DELETE CASCADE,
  user_uuid UUID NOT NULL REFERENCES users(uuid) ON DELETE CASCADE,
  role PROJECT_USER_ROLE DEFAULT 'viewer' NOT NULL,
  PRIMARY KEY (project_uuid, user_uuid)
);
CREATE INDEX project_user_roles_created_at_idx ON project_user_roles(created_at);
CREATE INDEX project_user_roles_updated_at_idx ON project_user_roles(updated_at);

SELECT manage_updated_at('project_user_roles');

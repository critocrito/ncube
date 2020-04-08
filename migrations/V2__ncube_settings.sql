CREATE TABLE IF NOT EXISTS setting (
  id INTEGER PRIMARY KEY,
  name TEXT NOT NULL,
  description TEXT NOT NULL,
  data_type TEXT NOT NULL,
  required INTEGER NOT NULL DEFAULT 0,
  UNIQUE (name)
);

INSERT INTO setting (
  name,
  description,
  data_type,
  required
) VALUES
('workspace_root', 'The directory where all local workspaces are stored.', 'alphanumeric', 1),
('name', 'The name of the local Ncube user.', 'alphanumeric', 0),
('email', 'The email address of the local Ncube user.', 'alphanumeric', 0)
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS ncube_config (
  id INTEGER PRIMARY KEY,
  setting INTEGER NOT NULL,
  value TEXT,
  UNIQUE (setting),
  FOREIGN KEY (setting) REFERENCES setting(id)
);
CREATE INDEX IF NOT EXISTS ncube_config_setting_idx ON ncube_config (setting);

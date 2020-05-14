CREATE TABLE IF NOT EXISTS workspace_database (
  id INTEGER PRIMARY KEY,
  workspace INTEGER NOT NULL REFERENCES workspace (id) ON DELETE CASCADE,
  kind TEXT NOT NULL,
  path TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  UNIQUE (workspace)
);
CREATE INDEX IF NOT EXISTS workspace_database_workspace_idx ON workspace_database(workspace);

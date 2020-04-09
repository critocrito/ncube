-- FIXME: Add 'PRAGMA foreign_keys = ON;' to enforce foreign key constraints.
INSERT INTO ncube_config (
  setting,
  value
) VALUES
(2, 'alice')
ON CONFLICT(setting) DO UPDATE SET value=excluded.value;

CREATE TABLE IF NOT EXISTS annotation (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  verification INTEGER NOT NULL REFERENCES verification(id),
  key TEXT NOT NULL,
  value JSON NOT NULL,
  name TEXT NOT NULL,
  note TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  UNIQUE (verification, key)
);
CREATE INDEX IF NOT EXISTS annotation_verification_idx ON annotation (verification);
CREATE INDEX IF NOT EXISTS annotation_key_idx ON annotation (key);

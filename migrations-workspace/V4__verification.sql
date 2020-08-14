CREATE TABLE IF NOT EXISTS verification (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  investigation INTEGER NOT NULL REFERENCES investigation(id),
  segment INTEGER NOT NULL REFERENCES segment(id),
  unit INTEGER NOT NULL REFERENCES unit(id),
  state JSON NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  UNIQUE (investigation, segment, unit)
);
CREATE INDEX IF NOT EXISTS verification_investigation_idx ON verification (investigation);
CREATE INDEX IF NOT EXISTS verification_segment_idx ON verification (segment);
CREATE INDEX IF NOT EXISTS verification_unit_idx ON verification (unit);

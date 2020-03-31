CREATE TABLE IF NOT EXISTS collection (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS data_segment (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  collection INTEGER NOT NULL,
  FOREIGN KEY (collection) REFERENCES collection(id)
);
CREATE INDEX IF NOT EXISTS data_segment_collection_idx ON data_segment (collection);

CREATE TABLE IF NOT EXISTS investigation (
  id INTEGER PRIMARY KEY,
  title TEXT NOT NULL,
  collection INTEGER NOT NULL,
  FOREIGN KEY (collection) REFERENCES collection(id)
);
CREATE INDEX IF NOT EXISTS investigation_collection_idx ON investigation (collection);

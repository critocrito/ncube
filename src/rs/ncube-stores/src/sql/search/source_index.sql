CREATE VIRTUAL TABLE source_fts
USING fts5(
  id,
  type,
  term,
  tokenize = 'porter unicode61 remove_diacritics 1',
  content='query',
  content_rowid='id'
);

CREATE TRIGGER source_ai AFTER INSERT ON query BEGIN
  INSERT INTO source_fts(
    id,
    type,
    term,
  ) VALUES (
    new.id,
    new.type,
    new.term
  );
END;

CREATE TRIGGER source_ad AFTER DELETE ON query BEGIN
  INSERT INTO source_fts(
    source_fts,
    id,
    type,
    term
  ) VALUES (
    'delete',
    old.id,
    old.type,
    old.term
  );
END;

CREATE TRIGGER source_au AFTER UPDATE ON query BEGIN
  INSERT INTO source_fts(
    source_fts,
    id,
    type,
    term
  ) VALUES (
    'delete',
    old.id,
    old.type,
    old.term
  );
  INSERT INTO source_fts(
    id,
    type,
    term
  ) VALUES (
    new.id,
    new.type,
    new.term
  );
END;

-- INSERT INTO source_fts SELECT id, type, term FROM query;

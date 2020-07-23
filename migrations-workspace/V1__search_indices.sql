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
    term
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

INSERT INTO source_fts SELECT id, type, term FROM query;

CREATE VIRTUAL TABLE unit_fts
USING fts5(
  id,
  body,
  title,
  description,
  author,
  tokenize = 'porter unicode61 remove_diacritics 1',
  content='unit',
  content_rowid='id'
);

CREATE TRIGGER unit_ai AFTER INSERT ON unit BEGIN
  INSERT INTO unit_fts(
    id,
    body,
    title,
    description,
    author
  ) VALUES (
    new.id,
    new.body,
    new.title,
    new.description,
    new.author
  );
END;

CREATE TRIGGER unit_ad AFTER DELETE ON unit BEGIN
  INSERT INTO unit_fts(
    unit_fts,
    id,
    body,
    title,
    description,
    author
  ) VALUES (
    'delete',
    old.id,
    old.body,
    old.title,
    old.description,
    old.author
  );
END;

CREATE TRIGGER unit_au AFTER UPDATE ON unit BEGIN
  INSERT INTO unit_fts(
    unit_fts,
    id,
    body,
    title,
    description,
    author
  ) VALUES (
    'delete',
    old.id,
    old.body,
    old.title,
    old.description,
    old.author
  );
  INSERT INTO unit_fts(
    id,
    body,
    title,
    description,
    author
  ) VALUES (
    new.id,
    new.body,
    new.title,
    new.description,
    new.author
  );
END;

INSERT INTO unit_fts SELECT id, body, title, description, author FROM unit;

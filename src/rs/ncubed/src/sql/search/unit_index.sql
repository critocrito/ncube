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

-- INSERT INTO unit_fts SELECT id, body, title, description, author FROM unit;

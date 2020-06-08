CREATE TEMPORARY TABLE tmp_table AS
SELECT
  id,
  email,
  password,
  name,
  created_at,
  updated_at
FROM account;

DROP TABLE account;

CREATE TABLE account (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  email TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  is_otp INTEGER NOT NULL DEFAULT 1,
  otp TEXT,
  symmetric_key BLOB,
  name TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL);

CREATE INDEX account_email_idx ON account (email);

INSERT INTO account
(id, email, password_hash, name, created_at, updated_at)
SELECT
  id,
  email,
  password,
  name,
  created_at,
  updated_at
FROM tmp_table;

DROP TABLE tmp_table;

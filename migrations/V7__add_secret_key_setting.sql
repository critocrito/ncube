ALTER TABLE setting
 ADD COLUMN restricted INTEGER NOT NULL DEFAULT 0;

INSERT INTO setting (
  name,
  description,
  data_type,
  required,
  restricted
) VALUES
('secret_key', 'The secret key that is used to sign JWT tokens.', 'alphanumeric', 1, 1)
ON CONFLICT DO NOTHING;

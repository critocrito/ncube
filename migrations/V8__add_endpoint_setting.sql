INSERT INTO setting (
  name,
  description,
  data_type,
  required,
  restricted
) VALUES
('endpoint', 'The URL of the HTTP endpoint.', 'alphanumeric', 0, 1)
ON CONFLICT DO NOTHING;

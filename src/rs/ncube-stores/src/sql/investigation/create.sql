INSERT INTO investigation (
  title,
  slug,
  description,
  methodology,
  created_at,
  updated_at
) VALUES
(?1, ?2, ?3, ?4, ?5, ?6)
ON CONFLICT(slug) DO NOTHING;

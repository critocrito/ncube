INSERT INTO segment (
  query,
  title,
  slug,
  created_at,
  updated_at
) VALUES
(?1, ?2, ?3, ?4, ?5)
ON CONFLICT(slug) DO NOTHING;

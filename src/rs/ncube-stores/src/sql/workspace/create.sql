INSERT INTO workspace (
  name,
  slug,
  description,
  kind,
  location,
  is_created,
  created_at,
  updated_at
) VALUES
(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
ON CONFLICT(slug) DO NOTHING;

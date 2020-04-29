SELECT
  id,
  name,
  slug,
  description,
  kind,
  location,
  created_at,
  updated_at
FROM workspace
WHERE slug = ?1;

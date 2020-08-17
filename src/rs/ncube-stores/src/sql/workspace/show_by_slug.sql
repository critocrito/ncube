SELECT
  w.id AS id,
  w.name AS name,
  w.slug AS slug,
  w.description AS description,
  w.kind AS kind,
  w.location AS location,
  w.created_at AS created_at,
  w.updated_at AS updated_at,
  wd.kind AS database,
  wd.path AS database_path,
  w.is_created AS is_created
FROM workspace AS w
LEFT JOIN workspace_database AS wd
     ON w.id = wd.workspace
WHERE slug = ?1;

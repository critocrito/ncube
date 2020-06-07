SELECT
  acc.id AS id,
  acc.email AS email,
  acc.name AS name,
  acc.created_at AS created_at,
  acc.updated_at AS updated_at,
  CASE
    WHEN acc.updated_at >= datetime('now', '-2 days', 'UTC') THEN acc.otp
    ELSE NULL
  END AS otp,
  w.slug AS workspace
FROM access a
  LEFT JOIN account acc ON a.account = acc.id
  LEFT JOIN workspace w ON a.workspace = w.id
WHERE acc.email = ?1 AND w.id = ?2;

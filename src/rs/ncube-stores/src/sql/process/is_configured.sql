WITH
  required_capabilities(cnt) AS (
    SELECT count(c.id)
      FROM capability c
     INNER JOIN dependency d ON c.dependency = d.id
     INNER JOIN workspace w ON c.workspace = w.id
     INNER JOIN process p ON c.process = p.id
      LEFT JOIN process_config pc ON c.id = pc.capability
     WHERE w.slug = ?1 AND p.key = ?2
  ),
  configured_capabilities(cnt) AS (
    SELECT count(pc.id)
      FROM process_config pc
     INNER JOIN capability c ON pc.capability = c.id
     INNER JOIN workspace w ON c.workspace = w.id
     INNER JOIN process p ON c.process = p.id
     WHERE w.slug = ?1 AND p.key = ?2
  )

SELECT (
  SELECT cnt FROM required_capabilities
) = (
  SELECT cnt FROM configured_capabilities
);

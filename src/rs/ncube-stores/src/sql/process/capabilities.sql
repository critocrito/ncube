SELECT d.name,
       d.key,
       d.description,
       d.kind,
       d.template,
       pc.value
  FROM capability c
 INNER JOIN dependency d ON c.dependency = d.id
 INNER JOIN workspace w ON c.workspace = w.id
  LEFT JOIN process_config pc ON c.id = pc.capability
 WHERE w.slug = ?1 AND c.process = ?2;

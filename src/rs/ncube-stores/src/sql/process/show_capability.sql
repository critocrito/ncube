SELECT c.id
  FROM capability c
  JOIN dependency d ON c.dependency = d.id
  JOIN workspace w ON c.workspace = w.id
 WHERE w.slug = ?1
   AND d.name = ?2;

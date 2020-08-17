SELECT DISTINCT s.id AS id,
       s.query AS query,
       s.title AS title,
       s.slug AS slug,
       s.created_at AS created_at,
       s.updated_at AS updated_at
  FROM verification v
  JOIN segment s ON v.segment = s.id
  JOIN investigation i ON v.investigation = i.id
 WHERE i.slug = ?1;

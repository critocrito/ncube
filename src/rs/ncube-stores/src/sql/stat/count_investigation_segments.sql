SELECT count(DISTINCT s.id)
  FROM verification v
  JOIN segment s ON v.segment = s.id
  JOIN investigation i ON v.investigation = i.id
 WHERE i.slug = ?1;

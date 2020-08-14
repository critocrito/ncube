SELECT count(DISTINCT v.id)
  FROM verification v
  JOIN investigation i ON v.investigation = i.id
 WHERE i.slug = ?1;

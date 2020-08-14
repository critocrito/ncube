SELECT count(DISTINCT v.id)
  FROM verification v
  JOIN investigation i ON v.investigation = i.id
 WHERE i.slug = ?1
   AND json_extract(v.state, '$.value') = 'verified_data';

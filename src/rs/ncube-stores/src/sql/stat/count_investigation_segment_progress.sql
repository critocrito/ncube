SELECT count(DISTINCT v.id)
  FROM verification v
  JOIN investigation i ON v.investigation = i.id
  JOIN segment s ON v.segment = s.id
 WHERE i.slug = ?1
   AND s.slug = ?2
   AND json_extract(v.state, '$.value') != 'verified_data';

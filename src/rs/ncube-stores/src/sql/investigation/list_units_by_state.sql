SELECT u.id AS id,
       u.source AS source,
       u.title AS title,
       (SELECT count(id) FROM download WHERE unit = u.id AND type = 'video') AS videos,
       (SELECT count(id) FROM download WHERE unit = u.id AND type = 'image') AS images,
       v.state AS state,
       v.id AS verification
  FROM verification v
  JOIN unit u ON v.unit = u.id
  JOIN segment s ON v.segment = s.id
  JOIN investigation i ON v.investigation = i.id
 WHERE i.id = ?1
   AND s.id = ?2
   AND json_extract(v.state, '$.value') = ?3;

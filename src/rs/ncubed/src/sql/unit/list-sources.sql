SELECT q.id as id,
       q.type AS type,
       q.term AS term
  FROM query_result qr
  LEFT JOIN query q on qr.query = q.id
 WHERE qr.unit = ?1;

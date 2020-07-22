SELECT id,
       label,
       description
  FROM query_tag
 WHERE label = ?1;

SELECT id,
       type,
       term
  FROM query
 WHERE type = ?1
   AND term = ?2;

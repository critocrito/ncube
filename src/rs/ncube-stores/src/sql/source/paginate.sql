SELECT id,
       type,
       term
  FROM query
 WHERE id NOT IN (
   SELECT id FROM query
   ORDER BY id DESC LIMIT ?1
 )
 ORDER BY id DESC LIMIT ?2;

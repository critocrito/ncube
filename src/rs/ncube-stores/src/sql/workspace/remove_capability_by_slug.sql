DELETE
  FROM capability
 WHERE workspace IN (
   SELECT id
     FROM workspace
    WHERE slug = ?1
 );

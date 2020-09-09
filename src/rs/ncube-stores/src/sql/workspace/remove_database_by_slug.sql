DELETE
  FROM workspace_database
 WHERE workspace IN (
   SELECT id
     FROM workspace
    WHERE slug = ?1
 );

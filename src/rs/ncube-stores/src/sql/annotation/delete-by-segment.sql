DELETE FROM annotation
 WHERE verification IN (
    SELECT id
      FROM verification
      WHERE segment = ?1
 );

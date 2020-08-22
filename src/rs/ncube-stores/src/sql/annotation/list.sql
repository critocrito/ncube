SELECT key,
       value,
       note,
       name,
       created_at,
       updated_at
  FROM annotation
 WHERE verification = ?1;

SELECT id,
       title,
       slug,
       description,
       process,
       created_at,
       updated_at
  FROM methodology
 WHERE slug = ?1;

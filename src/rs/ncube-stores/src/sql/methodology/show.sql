SELECT id,
       title,
       slug,
       description,
       process,
       initial_state,
       created_at,
       updated_at
  FROM methodology
 WHERE slug = ?1;

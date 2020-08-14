SELECT i.id AS id,
       m.initial_state AS initial_state
  FROM investigation i
  JOIN methodology m ON i.methodology = m.id
 WHERE i.slug = ?1;

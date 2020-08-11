SELECT i.id AS id,
       i.title AS title,
       i.slug AS slug,
       i.description AS description,
       i.created_at AS created_at,
       i.updated_at AS updated_at,
       m.slug AS methodology
  FROM investigation i
  JOIN methodology m ON i.methodology = m.id;

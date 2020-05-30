SELECT EXISTS(
  SELECT
    a.id
  FROM access a
    LEFT JOIN account ac ON a.account = ac.id
  WHERE a.workspace = ?1 AND
        ac.email = ?2
);

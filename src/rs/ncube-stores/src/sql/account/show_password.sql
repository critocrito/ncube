SELECT acc.password_hash AS password_hash
FROM access a
  LEFT JOIN account acc ON a.account = acc.id
  LEFT JOIN workspace w ON a.workspace = w.id
WHERE acc.email = ?1 AND w.id = ?2;

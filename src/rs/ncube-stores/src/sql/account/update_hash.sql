UPDATE account
   SET password_hash = ?1
 WHERE id = (
   SELECT acc.id AS id
     FROM access a
          LEFT JOIN account acc ON a.account = acc.id
          LEFT JOIN workspace w ON a.workspace = w.id
    WHERE acc.email = ?2 AND w.id = ?3
);

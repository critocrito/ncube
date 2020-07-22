UPDATE account
   SET password_hash = ?1,
       symmetric_key = ?2,
       updated_at = ?3,
       is_otp = 0,
       otp = NULL
 WHERE id = (
   SELECT acc.id AS id
     FROM access a
          LEFT JOIN account acc ON a.account = acc.id
          LEFT JOIN workspace w ON a.workspace = w.id
    WHERE acc.email = ?4 AND w.id = ?5
);

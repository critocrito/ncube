UPDATE query
SET type = ?2,
    term = ?3,
    updated_at = ?4
WHERE id = ?1;

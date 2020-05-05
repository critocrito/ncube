UPDATE workspace
SET name = ?1,
    slug = ?2,
    description = ?3,
    updated_at = ?6
WHERE slug = ?7;

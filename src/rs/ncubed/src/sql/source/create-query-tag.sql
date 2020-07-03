INSERT INTO query_tag (label, description)
VALUES (?1, ?2)
    ON CONFLICT (label) DO UPDATE
   SET updated_at = CURRENT_TIMESTAMP,
       description = excluded.description;

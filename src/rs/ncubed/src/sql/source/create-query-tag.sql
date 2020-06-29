INSERT INTO query_tag (query, name, value)
VALUES (?1, ?2, ?3)
    ON CONFLICT (query, name) DO UPDATE
   SET updated_at = CURRENT_TIMESTAMP,
       value = excluded.value;

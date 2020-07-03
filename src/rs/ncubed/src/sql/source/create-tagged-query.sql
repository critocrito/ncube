INSERT INTO tagged_query (query, query_tag)
VALUES (?1, ?2)
    ON CONFLICT (query, query_tag) DO UPDATE
   SET updated_at = CURRENT_TIMESTAMP;

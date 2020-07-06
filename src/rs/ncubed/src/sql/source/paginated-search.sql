SELECT q.id AS id,
       q.type AS type,
       q.term AS term
  FROM source_fts
  LEFT JOIN query q ON q.id = source_fts.id
 WHERE source_fts MATCH ?1
   AND q.id NOT IN (
   SELECT q.id
     FROM source_fts
     LEFT JOIN query q ON q.id = source_fts.id
    WHERE source_fts MATCH ?1
    ORDER BY bm25(source_fts), source_fts.id ASC LIMIT ?2
 )
 ORDER BY bm25(source_fts), source_fts.id ASC LIMIT ?3;

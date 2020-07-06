SELECT count(q.ROWID)
  FROM query q
  JOIN source_fts ON q.id = source_fts.id
 WHERE source_fts MATCH ?1;

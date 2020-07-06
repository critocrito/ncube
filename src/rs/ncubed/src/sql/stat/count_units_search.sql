SELECT count(u.ROWID)
  FROM unit u
  JOIN unit_fts ON u.id = unit_fts.id
 WHERE unit_fts MATCH ?1;

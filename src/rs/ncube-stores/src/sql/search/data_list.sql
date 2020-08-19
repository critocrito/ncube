SELECT u.id AS id
  FROM unit_fts
  LEFT JOIN unit u ON u.id = unit_fts.id
  LEFT JOIN tagged_unit tu ON tu.unit = unit_fts.id
  LEFT JOIN query_tag qt ON qt.id = tu.query_tag
 WHERE {};

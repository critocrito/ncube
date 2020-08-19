SELECT u.id AS id,
       u.id_hash AS id_hash,
       u.content_hash AS content_hash,
       u.source AS source,
       u.unit_id AS unit_id,
       u.body AS body,
       u.href AS href,
       u.author AS author,
       u.title AS title,
       u.description AS description,
       u.created_at AS created_at,
       u.fetched_at AS fetched_at,
       u.data AS data
  FROM unit_fts
  LEFT JOIN unit u ON u.id = unit_fts.id
  LEFT JOIN tagged_unit tu ON tu.unit = unit_fts.id
  LEFT JOIN query_tag qt ON qt.id = tu.query_tag
 WHERE {}
   AND u.id NOT IN (
   SELECT u.id
     FROM unit_fts
     LEFT JOIN unit u ON u.id = unit_fts.id
     LEFT JOIN tagged_unit tu ON tu.unit = unit_fts.id
     LEFT JOIN query_tag qt ON qt.id = tu.query_tag
    WHERE {}
    ORDER BY bm25(unit_fts), unit_fts.id ASC LIMIT ?1
 )
 ORDER BY bm25(unit_fts), unit_fts.id ASC LIMIT ?2;

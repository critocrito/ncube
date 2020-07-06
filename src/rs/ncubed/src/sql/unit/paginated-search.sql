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
       -- ,
       -- highlight(unit_fts, 1, '<span class="highlight">', '</span>') AS highlights_body,
       -- highlight(unit_fts, 2, '<span class="highlight">', '</span>') AS highlights_title,
       -- highlight(unit_fts, 3, '<span class="highlight">', '</span>') AS highlights_description,
       -- highlight(unit_fts, 4, '<span class="highlight">', '</span>') AS highlights_author,
       -- snippet(unit_fts, 1, '<span class="highlight">', '</span>', '...', '...') AS snippets
  FROM unit_fts
  LEFT JOIN unit u ON u.id = unit_fts.id
 WHERE unit_fts MATCH ?1
   AND u.id NOT IN (
   SELECT u.id
     FROM unit_fts
     LEFT JOIN unit u ON u.id = unit_fts.id
    WHERE unit_fts MATCH ?1
    ORDER BY bm25(unit_fts), unit_fts.id ASC LIMIT ?2
 )
 ORDER BY bm25(unit_fts), unit_fts.id ASC LIMIT ?3;

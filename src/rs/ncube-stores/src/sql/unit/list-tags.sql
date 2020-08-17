SELECT t.label,
       t.description
  FROM tagged_unit tu
  JOIN query_tag t ON tu.query_tag = t.id
 WHERE tu.unit = ?1;

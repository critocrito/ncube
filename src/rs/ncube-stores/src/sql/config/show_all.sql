SELECT
  s.name AS name,
  s.description AS description,
  s.required AS required,
  s.restricted AS restricted,
  nc.value AS value
FROM setting s
  LEFT JOIN ncube_config nc ON s.id = nc.setting

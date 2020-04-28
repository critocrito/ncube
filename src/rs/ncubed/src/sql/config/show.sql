SELECT
  s.name AS name,
  s.description AS description,
  s.required AS required,
  nc.value AS value
FROM ncube_config nc
  INNER JOIN setting s ON nc.setting = s.id;

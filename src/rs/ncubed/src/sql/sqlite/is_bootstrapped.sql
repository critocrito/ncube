WITH
  required_settings(cnt) AS (
    SELECT count(*)
    FROM setting
    WHERE required = 1
  ),
  configured_settings(cnt) AS (
    SELECT count(*)
    FROM ncube_config nc
      INNER JOIN setting s ON nc.setting = s.id
    WHERE s.required = 1
  )

SELECT (
  SELECT cnt FROM required_settings
) = (
  SELECT cnt FROM configured_settings
);

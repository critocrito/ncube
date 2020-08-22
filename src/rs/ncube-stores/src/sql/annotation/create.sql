INSERT INTO annotation (
  verification,
  key,
  value,
  name,
  note,
  created_at,
  updated_at
) VALUES
(?1, ?2, ?3, ?4, ?5, ?6, ?7)
  ON CONFLICT (verification, key) DO UPDATE
 SET value = excluded.value,
     note = excluded.note,
     name = excluded.name,
     updated_at = excluded.updated_at;

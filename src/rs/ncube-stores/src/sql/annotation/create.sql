INSERT INTO annotation (
  investigation,
  segment,
  unit,
  title,
  description,
  value,
  note,
  created_at,
  updated_at
) VALUES
(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
ON CONFLICT (investigation, segment, unit) DO NOTHING;

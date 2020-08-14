INSERT INTO verification (
  investigation,
  segment,
  unit,
  state,
  created_at,
  updated_at
) VALUES
(?1, ?2, ?3, ?4, ?5, ?6)
ON CONFLICT (investigation, segment, unit) DO NOTHING;

INSERT INTO query (
  type,
  term,
  created_at,
  updated_at
) VALUES (
  ?1,
  ?2,
  ?3,
  ?4
) ON CONFLICT (type, term) DO NOTHING;

INSERT INTO access (
  workspace,
  account,
  created_at,
  updated_at
) VALUES (?1, ?2, ?3, ?4)
ON CONFLICT (workspace, account) DO NOTHING;

INSERT INTO account (
  email,
  password_hash,
  otp,
  symmetric_key,
  created_at,
  updated_at
) VALUES (?1, ?2, ?3, ?4, ?5, ?6);

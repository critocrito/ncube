INSERT INTO process_config (capability, value)
VALUES (?1, ?2)
    ON CONFLICT (capability) DO UPDATE
   SET value = excluded.value;

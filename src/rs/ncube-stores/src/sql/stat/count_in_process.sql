SELECT count(DISTINCT id)
  FROM verification
 WHERE json_extract(state, '$.value') != 'verified_data';

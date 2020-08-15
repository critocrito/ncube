UPDATE verification
   SET state = ?4
 WHERE investigation = ?1
   AND segment = ?2
   AND unit = ?3;

UPDATE segment
   SET query = ?1,
       title = ?2,
       slug = ?3,
       updated_at = ?4
 WHERE slug = ?5;

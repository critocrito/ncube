INSERT INTO capability (
  dependency,
  process,
  workspace
) VALUES
((SELECT id FROM dependency WHERE name = 'Youtube API Key'), (SELECT id FROM process WHERE name = 'Youtube Video'), ?1),
((SELECT id FROM dependency WHERE name = 'Youtube API Key'), (SELECT id FROM process WHERE name = 'Youtube Channel'), ?1),
((SELECT id FROM dependency WHERE name = 'Twitter API Keys'), (SELECT id FROM process WHERE name = 'Twitter Tweet'), ?1),
((SELECT id FROM dependency WHERE name = 'Twitter API Keys'), (SELECT id FROM process WHERE name = 'Twitter Feed'), ?1)
ON CONFLICT DO NOTHING;

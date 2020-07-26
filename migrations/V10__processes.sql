CREATE TABLE IF NOT EXISTS process (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  name TEXT NOT NULL UNIQUE,
  description TEXT NOT NULL
);

INSERT INTO process (
  name,
  description
) VALUES
('Youtube Video', 'Fetch individual videos from Youtube.'),
('Youtube Channel', 'Fetch videos from Youtube channels.'),
('Twitter Tweet', 'Fetch individual tweets from Twitter.'),
('Twitter Feed', 'Fetch tweets from Twitter.')
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS dependency (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  kind TEXT NOT NULL,
  key TEXT NOT NULL,
  name TEXT NOT NULL UNIQUE,
  description TEXT NOT NULL,
  template JSON DEFAULT '{}'
);

INSERT INTO dependency (
  kind,
  key,
  name,
  description,
  template
) VALUES
('secret', 'youtube', 'Youtube API Key', 'Youtube API credentials.', '{"api_key":"Youtube API key"}'),
('secret', 'twitter', 'Twitter API Keys', 'Twitter OAuth 1.0a keys and tokens.', '{"consumer_key":"OAuth 1.0a Consumer Key","consumer_secret":"OAuth 1.0a Consumer Secret","access_token_key":"OAuth 1.0a Access Token","access_token_secret":"OAuth 1.0a Access Secret"}')
ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS capability (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  dependency INTEGER NOT NULL REFERENCES dependency(id),
  process INTEGER NOT NULL REFERENCES process(id),
  workspace INTEGER NOT NULL REFERENCES workspace(id),
  UNIQUE (dependency, process, workspace)
);
CREATE INDEX IF NOT EXISTS capability_dependency_idx ON capability (dependency);
CREATE INDEX IF NOT EXISTS capability_process_idx ON capability (process);
CREATE INDEX IF NOT EXISTS capability_workspace_idx ON capability (workspace);

CREATE TABLE IF NOT EXISTS process_config (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  capability INTEGER NOT NULL REFERENCES capability(id) UNIQUE,
  value JSON NOT NULL
);
CREATE INDEX IF NOT EXISTS process_config_capability_idx ON process_config (capability);

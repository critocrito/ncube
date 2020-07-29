ALTER TABLE process ADD COLUMN key TEXT NOT NULL DEFAULT "";
UPDATE process SET key = 'youtube_video' WHERE name = 'Youtube Video';
UPDATE process SET key = 'youtube_channel' WHERE name = 'Youtube Channel';
UPDATE process SET key = 'twitter_tweet' WHERE name = 'Twitter Tweet';
UPDATE process SET key = 'twitter_feed' WHERE name = 'Twitter Feed';

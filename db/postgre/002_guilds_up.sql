CREATE TABLE IF NOT EXISTS guilds (
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  owner_id TEXT NOT NULL REFERENCES users(id),
  banner_url TEXT,
  icon_url TEXT,
  default_channel_id TEXT
);

CREATE TABLE IF NOT EXISTS guild_members (
  guild_id TEXT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  roles JSONB NOT NULL DEFAULT '[]',
  joined_at TIMESTAMPTZ NOT NULL,

  identity_enabled BOOLEAN NOT NULL DEFAULT false,
  identity_display_name TEXT,
  identity_avatar_url TEXT,
  identity_bio TEXT,
  identity_show_global_username BOOLEAN NOT NULL DEFAULT true,

  PRIMARY KEY (guild_id, user_id)
);

CREATE TABLE IF NOT EXISTS categories (
  id TEXT PRIMARY KEY,
  guild_id TEXT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
  name TEXT NOT NULL,
  position INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS channels (
  id TEXT PRIMARY KEY,
  guild_id TEXT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
  category_id TEXT REFERENCES categories(id) ON DELETE SET NULL,
  name TEXT NOT NULL,
  position INTEGER NOT NULL,
  kind TEXT NOT NULL DEFAULT 'text',
  mode TEXT,
  user_limit INTEGER,
  bitrate INTEGER DEFAULT 64000
);

CREATE TABLE IF NOT EXISTS invites (
  invite_code TEXT PRIMARY KEY,
  guild_id TEXT NOT NULL REFERENCES guilds(id) ON DELETE CASCADE,
  creator_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  max_uses INTEGER NOT NULL DEFAULT 0,
  uses INTEGER NOT NULL DEFAULT 0,
  requires_approval BOOLEAN NOT NULL DEFAULT false,
  expires_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL
);

ALTER TABLE guilds
  ADD CONSTRAINT fk_guild_default_channel
  FOREIGN KEY (default_channel_id) REFERENCES channels(id)
  DEFERRABLE INITIALLY DEFERRED;

CREATE TABLE IF NOT EXISTS users (
  id TEXT PRIMARY KEY,
  username TEXT NOT NULL UNIQUE,
  email TEXT NOT NULL,
  password_hash TEXT NOT NULL,
  display_name TEXT,
  banner_url TEXT,
  avatar_url TEXT,
  profile_color TEXT,
  bio TEXT
);

CREATE TABLE IF NOT EXISTS user_settings (
  user_id TEXT PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
  locale TEXT NOT NULL DEFAULT 'en-US',
  developer_mode BOOLEAN NOT NULL DEFAULT true,
  notifications_active BOOLEAN NOT NULL DEFAULT true,
  theme_dark_mode TEXT NOT NULL DEFAULT 'system',
  theme_color TEXT NOT NULL DEFAULT 'default',
  theme_rounding TEXT NOT NULL DEFAULT 'default',
  theme_spacing TEXT NOT NULL DEFAULT 'default'
);

CREATE TABLE IF NOT EXISTS relationships (
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  target_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  status INTEGER NOT NULL,
  since TIMESTAMPTZ NOT NULL,
  PRIMARY KEY (user_id, target_id)
);

CREATE TABLE IF NOT EXISTS presence_presets (
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
  label TEXT NOT NULL,
  icon_kind TEXT NOT NULL,
  icon_value TEXT NOT NULL,
  timer_kind TEXT NOT NULL,
  timer_seconds INTEGER,
  preset_kind TEXT NOT NULL,
  process_name TEXT
);

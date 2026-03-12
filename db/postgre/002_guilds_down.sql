ALTER TABLE guilds DROP CONSTRAINT fk_guild_default_channel;

DROP TABLE IF EXISTS invites;
DROP TABLE IF EXISTS channels;
DROP TABLE IF EXISTS categories;
DROP TABLE IF EXISTS guild_members;
DROP TABLE IF EXISTS guilds;

CREATE KEYSPACE IF NOT EXISTS chat
WITH replication = {
    'class': 'SimpleStrategy',
    'replication_factor': 1
};

USE chat;

CREATE TABLE IF NOT EXISTS messages (
    channel_id        UUID,
    created_at        TIMESTAMP,
    id                UUID,
    author_id         UUID,
    content           TEXT,
    edited_at         TIMESTAMP,

    PRIMARY KEY ((channel_id), created_at, id)
) WITH CLUSTERING ORDER BY (created_at DESC, id DESC);

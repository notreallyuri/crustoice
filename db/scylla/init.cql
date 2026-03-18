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
    deleted           BOOLEAN,
    thread_id         UUID,

    PRIMARY KEY ((channel_id), created_at, id)
) WITH CLUSTERING ORDER BY (created_at DESC, id DESC);
    
CREATE INDEX messages_deleted_idx ON messages (deleted);
    
CREATE INDEX messages_thread_id_idx ON messages (thread_id);

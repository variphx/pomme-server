CREATE TABLE
    chat_participants (
        chat_id BIGINT NOT NULL REFERENCES chats,
        user_id BIGINT NOT NULL REFERENCES users,
        PRIMARY KEY (chat_id, user_id)
    );
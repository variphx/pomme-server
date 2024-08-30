INSERT INTO
    chats (chat_name)
VALUES
    ($1)
RETURNING
    chat_id
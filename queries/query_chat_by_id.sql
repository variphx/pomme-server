SELECT
    chat_id,
    chat_name,
    updated_at
FROM
    chats
WHERE
    chat_id = $1
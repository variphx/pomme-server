SELECT
    *
FROM
    (
        SELECT
            chats.chat_id,
            chats.chat_name,
            chats.updated_at
        FROM
            chats,
            chat_participants
        WHERE
            chats.chat_id = chat_participants.chat_id
            AND chat_participants.user_id = $1
            AND chats.chat_name LIKE $2
    )
ORDER BY
    updated_at
LIMIT
    $3
OFFSET
    $4
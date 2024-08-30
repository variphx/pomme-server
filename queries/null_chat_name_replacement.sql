SELECT
    users.username
FROM
    chat_participants,
    chats,
    users
WHERE
    chat_participants.chat_id = chats.chat_id
    AND chat_participants.user_id = users.user_id
    AND users.user_id <> $1
ORDER BY
    users.username
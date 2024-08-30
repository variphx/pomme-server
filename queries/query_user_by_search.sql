SELECT
    *
FROM
    (
        SELECT
            user_id,
            username,
            email
        FROM
            users
        WHERE
            username LIKE $1
    )
ORDER BY
    LENGTH(username)
LIMIT
    $2
OFFSET
    $3
CREATE TABLE
    users (
        user_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
        username TEXT NOT NULL,
        email TEXT NOT NULL,
        password_hash TEXT NOT NULL
    );
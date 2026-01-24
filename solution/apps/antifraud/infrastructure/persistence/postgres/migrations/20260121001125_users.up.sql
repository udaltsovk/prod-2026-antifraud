DO
$$
BEGIN
IF NOT EXISTS (
    SELECT
        1
    FROM
        pg_type
    WHERE
        typname = 'user_gender'
) THEN CREATE TYPE user_gender AS ENUM ('MALE', 'FEMALE');

END IF;

IF NOT EXISTS (
    SELECT
        1
    FROM
        pg_type
    WHERE
        typname = 'user_marital_status'
) THEN CREATE TYPE user_marital_status AS ENUM (
    'SINGLE',
    'MARRIED',
    'DIVORCED',
    'WIDOWED'
);

END IF;

IF NOT EXISTS (
    SELECT
        1
    FROM
        pg_type
    WHERE
        typname = 'user_role'
) THEN CREATE TYPE user_role AS ENUM ('ADMIN', 'USER');

END IF;

END
$$
;

CREATE TABLE IF NOT EXISTS users (
    id uuid NOT NULL PRIMARY KEY,
    email text NOT NULL UNIQUE,
    full_name text NOT NULL,
    password_hash text NOT NULL,
    age smallint,
    gender user_gender,
    marital_status user_marital_status,
    region text,
    role user_role NOT NULL,
    is_active bool NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS users_email_idx ON users (email);

CREATE INDEX IF NOT EXISTS users_created_at_idx ON users (created_at);

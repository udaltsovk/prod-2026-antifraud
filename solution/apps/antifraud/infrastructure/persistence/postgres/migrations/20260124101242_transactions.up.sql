DO
$$
BEGIN
IF NOT EXISTS (
    SELECT
        1
    FROM
        pg_type
    WHERE
        typname = 'transaction_verdict'
) THEN CREATE TYPE transaction_verdict AS ENUM ('ACCEPTED', 'DECLINED');

END IF;

IF NOT EXISTS (
    SELECT
        1
    FROM
        pg_type
    WHERE
        typname = 'transaction_channel'
) THEN CREATE TYPE transaction_channel AS ENUM (
    'WEB',
    'MOBILE',
    'POS',
    'OTHER'
);

END IF;

IF NOT EXISTS (
    SELECT
        1
    FROM
        pg_type
    WHERE
        typname = 'transaction_location'
) THEN CREATE TYPE transaction_location AS (
    country text,
    city text,
    latitude real,
    longitude real
);

END IF;

END
$$
;

CREATE TABLE IF NOT EXISTS transactions(
    id uuid NOT NULL PRIMARY KEY,
    user_id uuid NOT NULL REFERENCES users (id) ON DELETE CASCADE,
    amount double precision NOT NULL,
    currency text NOT NULL,
    verdict transaction_verdict NOT NULL,
    merchant_id text,
    merchant_category_code text,
    specified_timestamp timestamptz NOT NULL,
    ip_address inet,
    device_id text,
    channel transaction_channel,
    location transaction_location NOT NULL,
    metadata jsonb,
    created_at timestamptz NOT NULL
);

CREATE INDEX transactions_user_id_verdict_created_at_idx ON transactions (user_id, verdict, created_at DESC);

INSERT INTO
    transactions (
        id,
        user_id,
        amount,
        currency,
        verdict,
        merchant_id,
        merchant_category_code,
        specified_timestamp,
        ip_address,
        device_id,
        channel,
        location,
        metadata,
        created_at
    )
VALUES
    (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6,
        $7,
        $8,
        $9,
        $10,
        $11,
        $12,
        $13,
        NOW()
    )
RETURNING
    id,
    user_id,
    amount,
    currency,
    verdict AS "verdict: _",
    merchant_id,
    merchant_category_code,
    specified_timestamp,
    ip_address AS "ip_address: _",
    device_id,
    channel AS "channel: _",
    location AS "location: _",
    metadata,
    created_at

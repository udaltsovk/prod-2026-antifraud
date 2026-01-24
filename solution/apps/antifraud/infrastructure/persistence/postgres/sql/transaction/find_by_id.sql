SELECT
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
FROM
    transactions
WHERE
    id = $1

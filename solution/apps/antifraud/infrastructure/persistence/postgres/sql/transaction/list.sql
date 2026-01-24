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
    (
        $1::uuid IS NULL
        OR user_id = $1
    )
    AND (
        $2::transaction_verdict IS NULL
        OR verdict = $2
    )
    AND $3 <= created_at
    AND created_at < $4
ORDER BY
    created_at DESC
LIMIT
    $5 OFFSET $6

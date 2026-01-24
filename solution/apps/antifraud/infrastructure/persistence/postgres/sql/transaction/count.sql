SELECT
    COUNT(*)
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
    AND created_at >= $3
    AND created_at < $4

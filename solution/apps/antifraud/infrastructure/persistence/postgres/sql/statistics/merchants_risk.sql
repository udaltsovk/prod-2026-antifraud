WITH filtered_tx AS (
    SELECT
        merchant_id,
        merchant_category_code,
        amount,
        verdict
    FROM
        transactions
    WHERE
        specified_timestamp >= $1
        AND specified_timestamp < $2
        AND merchant_id IS NOT NULL
        AND (
            $3::text IS NULL
            OR merchant_category_code = $3
        )
)
SELECT
    merchant_id AS "merchant_id!",
    merchant_category_code,
    COUNT(*)::bigint AS "tx_count!",
    COALESCE(SUM(amount), 0)::double precision AS "gmv!",
    COALESCE(
        SUM(
            CASE
                WHEN verdict = 'DECLINED' THEN 1
                ELSE 0
            END
        )::float / NULLIF(COUNT(*), 0),
        0
    )::real AS "decline_rate!"
FROM
    filtered_tx
GROUP BY
    merchant_id,
    merchant_category_code
ORDER BY
    "decline_rate!" DESC,
    "tx_count!" DESC
LIMIT
    $4;

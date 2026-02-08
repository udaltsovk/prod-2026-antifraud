WITH base_tx AS (
    SELECT
        specified_timestamp AT TIME ZONE $3 AS ts_local,
        verdict,
        amount,
        channel
    FROM
        transactions
    WHERE
        specified_timestamp >= $1
        AND specified_timestamp < $2
        AND (
            $4::transaction_channel IS NULL
            OR channel = $4
        )
),
bucketed AS (
    SELECT
        date_trunc($5, ts_local) AS bucket_start_local,
        verdict,
        amount
    FROM
        base_tx
)
SELECT
    bucket_start_local AT TIME ZONE $3 AS "bucket_start!",
    COUNT(*)::bigint AS "tx_count!",
    COALESCE(SUM(amount), 0)::double precision AS "gmv!",
    COALESCE(
        SUM(
            CASE
                WHEN verdict = 'APPROVED' THEN 1
                ELSE 0
            END
        )::float / NULLIF(COUNT(*), 0),
        0
    )::real AS "approval_rate!",
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
    bucketed
GROUP BY
    bucket_start_local
ORDER BY
    bucket_start_local;

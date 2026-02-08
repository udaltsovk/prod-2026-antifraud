WITH base_tx AS (
    SELECT
        verdict,
        amount,
        merchant_id,
        merchant_category_code
    FROM
        transactions
    WHERE
        specified_timestamp >= $1
        AND specified_timestamp < $2
),
overview AS (
    SELECT
        COUNT(*)::bigint AS volume,
        COALESCE(SUM(amount), 0)::double precision AS gmv,
        COALESCE(
            SUM(
                CASE
                    WHEN verdict = 'APPROVED' THEN 1
                    ELSE 0
                END
            )::float / NULLIF(COUNT(*), 0),
            0
        )::real AS approval_rate,
        COALESCE(
            SUM(
                CASE
                    WHEN verdict = 'DECLINED' THEN 1
                    ELSE 0
                END
            )::float / NULLIF(COUNT(*), 0),
            0
        )::real AS decline_rate
    FROM
        base_tx
),
merchant_stats AS (
    SELECT
        merchant_id,
        merchant_category_code,
        COUNT(*)::bigint AS tx_count,
        COALESCE(SUM(amount), 0)::double precision AS gmv,
        COALESCE(
            SUM(
                CASE
                    WHEN verdict = 'DECLINED' THEN 1
                    ELSE 0
                END
            )::float / NULLIF(COUNT(*), 0),
            0
        )::real AS decline_rate
    FROM
        base_tx
    WHERE
        merchant_id IS NOT NULL
    GROUP BY
        merchant_id,
        merchant_category_code
),
top_merchants AS (
    SELECT
        COALESCE(
            array_agg(
                ROW(
                    merchant_id,
                    merchant_category_code,
                    tx_count,
                    gmv,
                    decline_rate
                )::merchant_risk_stats
                ORDER BY
                    decline_rate DESC,
                    tx_count DESC
            ),
            ARRAY []::merchant_risk_stats []
        ) AS merchants
    FROM
        (
            SELECT
                *
            FROM
                merchant_stats
            ORDER BY
                decline_rate DESC,
                tx_count DESC
            LIMIT
                10
        ) t
)
SELECT
    $1 AS "from!",
    $2 AS "to!",
    overview.volume AS "volume!",
    overview.gmv AS "gmv!",
    overview.approval_rate AS "approval_rate!",
    overview.decline_rate AS "decline_rate!",
    top_merchants.merchants AS "top_risk_merchants!: _"
FROM
    overview
    CROSS JOIN top_merchants;

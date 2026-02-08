WITH filtered_tx AS (
    SELECT
        tr.transaction_id,
        tr.rule_id,
        tr.rule_name,
        tr.matched,
        tx.user_id,
        tx.merchant_id,
        tx.verdict
    FROM
        fraud_rule_results tr
        JOIN transactions tx ON tr.transaction_id = tx.id
    WHERE
        tx.specified_timestamp >= $1
        AND tx.specified_timestamp < $2
),
agg AS (
    SELECT
        rule_id,
        rule_name,
        COUNT(*)::bigint AS "matches!",
        COUNT(DISTINCT user_id)::bigint AS "unique_users!",
        COUNT(DISTINCT merchant_id)::bigint AS "unique_merchants!",
        COALESCE(
            SUM(
                CASE
                    WHEN verdict = 'DECLINED' THEN 1
                    ELSE 0
                END
            )::float / NULLIF(COUNT(*), 0),
            0
        )::real AS "share_of_declines!"
    FROM
        filtered_tx
    WHERE
        matched = TRUE
    GROUP BY
        rule_id,
        rule_name
)
SELECT
    *
FROM
    agg
ORDER BY
    "share_of_declines!" DESC,
    "matches!" DESC
LIMIT
    $3;

WITH tx_24h AS (
    SELECT
        id AS tx_id,
        amount AS amount_24h,
        device_id AS device_id_24h,
        ip_address AS ip_address_24h,
        (location).city AS city_24h,
        specified_timestamp AS ts_24h
    FROM
        transactions
    WHERE
        user_id = $1
        AND specified_timestamp >= NOW() - INTERVAL '24 hours'
),
tx_30d AS (
    SELECT
        id AS tx_id,
        verdict AS verdict_30d
    FROM
        transactions
    WHERE
        user_id = $1
        AND specified_timestamp >= NOW() - INTERVAL '30 days'
)
SELECT
    $1 AS "user_id!",
    COALESCE(COUNT(tx_24h.tx_id), 0)::bigint AS "tx_count_24h!",
    COALESCE(SUM(tx_24h.amount_24h), 0)::double precision AS "gmv_24h!",
    COALESCE(COUNT(DISTINCT tx_24h.device_id_24h), 0)::bigint AS "distinct_devices_24h!",
    COALESCE(COUNT(DISTINCT tx_24h.ip_address_24h), 0)::bigint AS "distinct_ips_24h!",
    COALESCE(COUNT(DISTINCT tx_24h.city_24h), 0)::bigint AS "distinct_cities_24h!",
    COALESCE(
        SUM(
            CASE
                WHEN tx_30d.verdict_30d = 'DECLINED' THEN 1
                ELSE 0
            END
        )::float / NULLIF(COUNT(tx_30d.tx_id), 0),
        0
    )::real AS "decline_rate_30d!",
    NULL::timestamptz AS last_seen_at
FROM
    tx_24h
    FULL OUTER JOIN tx_30d ON tx_24h.tx_id = tx_30d.tx_id;

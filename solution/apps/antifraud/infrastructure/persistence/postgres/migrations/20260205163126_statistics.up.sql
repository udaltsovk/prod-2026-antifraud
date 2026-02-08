DO
$$
BEGIN
IF NOT EXISTS (
    SELECT
        1
    FROM
        pg_type
    WHERE
        typname = 'merchant_risk_stats'
) THEN CREATE TYPE merchant_risk_stats AS (
    merchant_id text,
    merchant_category_code text,
    tx_count bigint,
    gmv double precision,
    decline_rate real
);

END IF;

END
$$
;

SELECT
    id,
    name,
    description,
    dsl_expression,
    enabled,
    priority,
    created_at,
    updated_at
FROM
    fraud_rules
WHERE
    (
        $1::bool IS NULL
        OR enabled = $1
    )
ORDER BY
    priority

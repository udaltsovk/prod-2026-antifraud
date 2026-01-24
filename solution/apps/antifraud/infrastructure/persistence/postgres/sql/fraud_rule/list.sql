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
ORDER BY
    priority

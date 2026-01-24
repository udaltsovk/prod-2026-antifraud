UPDATE
    fraud_rules
SET
    name = $2,
    description = $3,
    dsl_expression = $4,
    enabled = $5,
    priority = $6,
    updated_at = NOW()
WHERE
    id = $1
RETURNING
    id,
    name,
    description,
    dsl_expression,
    enabled,
    priority,
    created_at,
    updated_at

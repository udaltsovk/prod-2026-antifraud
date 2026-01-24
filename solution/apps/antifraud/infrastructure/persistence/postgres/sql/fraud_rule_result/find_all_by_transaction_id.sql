SELECT
    transaction_id,
    rule_id,
    rule_name,
    priority,
    matched,
    description
FROM
    fraud_rule_results
WHERE
    transaction_id = $1
ORDER BY
    priority

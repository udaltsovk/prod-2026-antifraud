INSERT INTO
    fraud_rule_results (
        transaction_id,
        rule_id,
        rule_name,
        priority,
        matched,
        description
    )
VALUES
    (
        $1,
        $2,
        $3,
        $4,
        $5,
        $6
    )
RETURNING
    transaction_id,
    rule_id,
    rule_name,
    priority,
    matched,
    description

CREATE TABLE IF NOT EXISTS fraud_rule_results (
    transaction_id uuid NOT NULL REFERENCES transactions (id) ON DELETE CASCADE,
    rule_id uuid NOT NULL REFERENCES fraud_rules (id) ON DELETE CASCADE,
    rule_name text NOT NULL,
    priority bigint NOT NULL,
    matched bool NOT NULL,
    description text NOT NULL,
    PRIMARY KEY (transaction_id, rule_id)
);

CREATE INDEX IF NOT EXISTS fraud_rule_results_transaction_id_idx ON fraud_rule_results (transaction_id);

CREATE INDEX IF NOT EXISTS fraud_rule_results_priority_idx ON fraud_rule_results (priority);

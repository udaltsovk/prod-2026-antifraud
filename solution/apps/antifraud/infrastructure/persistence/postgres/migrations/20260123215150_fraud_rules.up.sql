CREATE TABLE IF NOT EXISTS fraud_rules (
    id uuid NOT NULL PRIMARY KEY,
    name text NOT NULL UNIQUE,
    description text,
    dsl_expression text NOT NULL,
    enabled bool NOT NULL,
    priority bigint NOT NULL,
    created_at timestamptz NOT NULL,
    updated_at timestamptz NOT NULL
);

CREATE UNIQUE INDEX IF NOT EXISTS fraud_rules_name_idx ON fraud_rules (name);

CREATE INDEX IF NOT EXISTS fraud_rules_priority_idx ON fraud_rules (priority);

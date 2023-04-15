CREATE TABLE IF NOT EXISTS transactions
(
    transaction_id    TEXT      NOT NULL,
    payer_address     TEXT      NOT NULL,
    receiver_address  TEXT      NOT NULL,
    token_address     TEXT      NOT NULL,
    amount            BIGINT    NOT NULL,
    cumulative_amount BIGINT    NOT NULL,
    created_at        TIMESTAMP NOT NULL,

    CONSTRAINT pk_transactions PRIMARY KEY (transaction_id)
)
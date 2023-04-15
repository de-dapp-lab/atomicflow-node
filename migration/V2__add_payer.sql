CREATE TABLE IF NOT EXISTS payers
(
    address     TEXT NOT NULL,
    evm_address TEXT NOT NULL,
    assets      TEXT NOT NULL,

    CONSTRAINT pk_payers PRIMARY KEY (address)
    );

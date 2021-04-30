-- Your SQL goes here
CREATE TABLE IF NOT EXISTS accounts (
  id VARCHAR(36) PRIMARY KEY,
  email VARCHAR(256) NOT NULL,
  password_hash VARCHAR(256) NOT NULL
);

ALTER TABLE accounts ADD UNIQUE uidx_accounts_01 (email);
ALTER TABLE accounts ADD INDEX idx_accounts_01 (email);

CREATE TABLE IF NOT EXISTS accounts (
  id VARCHAR(36) PRIMARY KEY,
  user_id VARCHAR(36) NOT NULL,
  email VARCHAR(256) NOT NULL,
  password_hash VARCHAR(256) NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE UNIQUE INDEX uidx_accounts_01 ON accounts (email);
ALTER TABLE accounts ADD FOREIGN KEY (user_id) REFERENCES users(id);

CREATE TRIGGER update_trigger BEFORE UPDATE ON accounts FOR EACH ROW EXECUTE PROCEDURE set_update_time();

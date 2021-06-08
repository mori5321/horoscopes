CREATE TABLE IF NOT EXISTS users (
  id VARCHAR(36) PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE FUNCTION set_update_time() RETURNS OPAQUE AS '
  begin
    new.updated_at := ''now'';
    return new;
  end;
' LANGUAGE plpgsql;

CREATE TRIGGER update_trigger BEFORE UPDATE ON users FOR EACH ROW EXECUTE PROCEDURE set_update_time();


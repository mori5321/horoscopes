CREATE TABLE IF NOT EXISTS users_organizations (
  user_id VARCHAR(36) NOT NULL,
  organization_id VARCHAR(36) NOT NULL,
  FOREIGN KEY (user_id) REFERENCES users(id),
  FOREIGN KEY (organization_id) REFERENCES organizations(id),
  PRIMARY KEY (user_id, organization_id)
);

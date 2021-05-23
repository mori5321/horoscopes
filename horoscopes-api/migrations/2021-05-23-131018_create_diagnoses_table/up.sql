CREATE TABLE IF NOT EXISTS diagnoses (
  id VARCHAR(36) PRIMARY KEY,
  title VARCHAR(256) NOT NULL,
  organization_id VARCHAR(36) NOT NULL,
  FOREIGN KEY fk_organizations_id(organization_id) REFERENCES organizations(id)
);

CREATE TABLE IF NOT EXISTS questions (
  id VARCHAR(36) PRIMARY KEY,
  text VARCHAR(256) NOT NULL,
  diagnosis_id VARCHAR(36) NOT NULL,
  question_type INT NOT NULL,
  FOREIGN KEY fk_diagnoses_id(diagnosis_id) REFERENCES diagnoses(id)
);

CREATE TABLE IF NOT EXISTS answer_frames (
  id VARCHAR(36) PRIMARY KEY,
  text VARCHAR(256) NOT NULL,
  question_id VARCHAR(36) NOT NULL,
  FOREIGN KEY fk_questions_id(question_id) REFERENCES questions(id)
);

-- Your SQL goes here
CREATE TABLE players (
  id VARCHAR(255) NOT NULL PRIMARY KEY,
  name VARCHAR(255) NOT NULL,
  UNIQUE (name)
)
-- Lots of repeated accounts here; ok to ignore for now.

CREATE TABLE(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  signer VARCHAR(45) NOT NULL,
  spl_account VARCHAR(45) NOT NULL,
  spl_program VARCHAR(45) NOT NULL,
  token_program VARCHAR(45) NOT NULL,
);

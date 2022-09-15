CREATE TABLE target_parent_programs(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  name VARCHAR(20). 
  accounts_table VARCHAR(45) NOT NULL,
  parent_program VARCHAR(45) NOT NULL UNIQUE,
);

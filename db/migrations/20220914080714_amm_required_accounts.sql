CREATE TABLE solana(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program VARCHAR(45) NOT NULL,
  pool_program VARCHAR(45) NOT NULL UNIQUE,
  pool_authority VARCHAR(45) NOT NULL,
  user_authority VARCHAR(45),
  user_source VARCHAR(45),
  pool_source VARCHAR(45) NOT NULL,
  pool_destination VARCHAR(45) NOT NULL,
  user_destination VARCHAR(45),
  pool_mint VARCHAR(45) NOT NULL UNIQUE,
  pool_fee VARCHAR(45) NOT NULL UNIQUE,
  token_program VARCHAR(45) NOT NULL,
);

CREATE TABLE orca(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program VARCHAR(45) NOT NULL,
  pool_program VARCHAR(45) NOT NULL UNIQUE,
  pool_authority VARCHAR(45) NOT NULL,
  user_authority VARCHAR(45),
  user_source VARCHAR(45),
  pool_source VARCHAR(45) NOT NULL,
  pool_destination VARCHAR(45) NOT NULL,
  pool_destination VARCHAR(45),
  pool_mint VARCHAR(45) NOT NULL UNIQUE,
  pool_fee VARCHAR(45) NOT NULL UNIQUE,
  token_program VARCHAR(45) NOT NULL,
);

CREATE TABLE orcav2(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program VARCHAR(45) NOT NULL,
  pool_program VARCHAR(45) NOT NULL UNIQUE,
  pool_authority VARCHAR(45) NOT NULL,
  user_authority VARCHAR(45),
  user_source VARCHAR(45),
  pool_source VARCHAR(45) NOT NULL,
  pool_destination VARCHAR(45) NOT NULL,
  pool_destination VARCHAR(45),
  pool_mint VARCHAR(45) NOT NULL UNIQUE,
  pool_fee VARCHAR(45) NOT NULL UNIQUE,
  token_program VARCHAR(45) NOT NULL,
);

-- Saros is likely dead
CREATE TABLE saros(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program VARCHAR(45) NOT NULL,
  pool_program VARCHAR(45) NOT NULL UNIQUE,
  pool_authority VARCHAR(45) NOT NULL,
  user_authority VARCHAR(45),
  user_source VARCHAR(45),
  pool_source VARCHAR(45) NOT NULL,
  pool_destination VARCHAR(45) NOT NULL,
  user_destination VARCHAR(45),
  pool_mint VARCHAR(45) NOT NULL UNIQUE,
  pool_fee VARCHAR(45) NOT NULL UNIQUE,
  token_program VARCHAR(45) NOT NULL,
);

CREATE TABLE saber(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program VARCHAR(45) NOT NULL,
  pool_program VARCHAR(45) NOT NULL UNIQUE,
  pool_authority VARCHAR(45) NOT NULL,
  user_authority VARCHAR(45),
  user_source VARCHAR(45),
  pool_source VARCHAR(45) NOT NULL,
  pool_destination VARCHAR(45) NOT NULL,
  user_destination VARCHAR(45),
  admin_destination VARCHAR(45) NOT NULL,
  token_program VARCHAR(45) NOT NULL,
);

-- point refund_to @ token program
CREATE TABLE step(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program VARCHAR(45) NOT NULL,
  pool_program VARCHAR(45) NOT NULL UNIQUE,
  pool_authority VARCHAR(45) NOT NULL,
  user_authority VARCHAR(45),
  user_source VARCHAR(45),
  pool_source VARCHAR(45) NOT NULL,
  pool_destination VARCHAR(45) NOT NULL,
  user_destination VARCHAR(45),
  pool_mint VARCHAR(45) NOT NULL UNIQUE,
  pool_fee VARCHAR(45) NOT NULL UNIQUE,
  refund_to VARCHAR(45) NOT NULL,
  token_program VARCHAR(45) NOT NULL,
);

CREATE TABLE stepn(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program VARCHAR(45) NOT NULL,
  pool_program VARCHAR(45) NOT NULL UNIQUE,
  pool_authority VARCHAR(45) NOT NULL,
  user_authority VARCHAR(45),
  user_source VARCHAR(45),
  pool_source VARCHAR(45) NOT NULL,
  pool_destination VARCHAR(45) NOT NULL,
  user_destination VARCHAR(45),
  pool_mint VARCHAR(45) NOT NULL UNIQUE,
  pool_fee VARCHAR(45) NOT NULL UNIQUE,
  refund_to VARCHAR(45) NOT NULL,
);

CREATE TABLE aldrin(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program VARCHAR(45) NOT NULL,
  pool_program VARCHAR(45) NOT NULL UNIQUE,
  pool_authority VARCHAR(45) NOT NULL,
  pool_mint VARCHAR(45) NOT NULL UNIQUE,
  pool_base_token_vault VARCHAR(45) NOT NULL,
  pool_quote_token_vault VARCHAR(45) NOT NULL,
  user_authority VARCHAR(45),
  user_base_token_account VARCHAR(45),
  user_quote_token_account VARCHAR(45),
  curve VARCHAR(45) NOT NULL,
  pool_fee VARCHAR(45) NOT NULL,
);

-- These keys need reviewing. `rent` points to sysvar rent
CREATE TABLE serum(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program VARCHAR(45) NOT NULL,
  market VARCHAR(45) NOT NULL,
  market_open_orders VARCHAR(45) NOT NULL,
  market_request_queue VARCHAR(45) NOT NULL,
  market_event_queue VARCHAR(45) NOT NULL,
  market_bids VARCHAR(45) NOT NULL,
  market_asks VARCHAR(45) NOT NULL,
  market_order_payer_token_account VARCHAR(45) NOT NULL,
  market_coin_vault VARCHAR(45) NOT NULL,
  market_pc_vault VARCHAR(45) NOT NULL,
  market_vault_signer VARCHAR(45) NOT NULL,
  market_coin_wallet VARCHAR(45) NOT NULL,
  user_authority VARCHAR(45),
  user_pc_wallet TEXT,
  dex_program VARCHAR(45) NOT NULL,
  token_program VARCHAR(45) NOT NULL,
  rent VARCHAR(45) NOT NULL,
);


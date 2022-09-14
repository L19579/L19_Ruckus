CREATE TABLE solana(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program TEXT NOT  NULL,
  pool_program TEXT NOT  NULL,
  pool_authority TEXT NOT NULL,
  user_authority TEXT,
  user_source TEXT,
  pool_source TEXT NOT NULL,
  pool_destination TEXT NOT NULL,
  user_destination TEXT,
  pool_mint TEXT NOT NULL,
  pool_fee TEXT NOT NULL,
  token_program TEXT NOT NULL,
);

CREATE TABLE orca(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program TEXT NOT  NULL,
  pool_program TEXT NOT  NULL,
  pool_authority TEXT NOT NULL,
  user_authority TEXT,
  user_source TEXT,
  pool_source TEXT NOT NULL,
  pool_destination TEXT NOT NULL,
  pool_destination TEXT,
  pool_mint TEXT NOT NULL,
  pool_fee TEXT NOT NULL,
  token_program TEXT NOT NULL,
);

CREATE TABLE orcav2(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program TEXT NOT  NULL,
  pool_program TEXT NOT  NULL,
  pool_authority TEXT NOT NULL,
  user_authority TEXT,
  user_source TEXT,
  pool_source TEXT NOT NULL,
  pool_destination TEXT NOT NULL,
  pool_destination TEXT,
  pool_mint TEXT NOT NULL,
  pool_fee TEXT NOT NULL,
  token_program TEXT NOT NULL,
);

-- Saros is likely dead
CREATE TABLE saros(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program TEXT NOT  NULL,
  pool_program TEXT NOT  NULL,
  pool_authority TEXT NOT NULL,
  user_authority TEXT,
  user_source TEXT,
  pool_source TEXT NOT NULL,
  pool_destination TEXT NOT NULL,
  user_destination TEXT,
  pool_mint TEXT NOT NULL,
  pool_fee TEXT NOT NULL,
  token_program TEXT NOT NULL,
);

CREATE TABLE saber(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program TEXT NOT  NULL,
  pool_program TEXT NOT  NULL,
  pool_authority TEXT NOT NULL,
  user_authority TEXT,
  user_source TEXT,
  pool_source TEXT NOT NULL,
  pool_destination TEXT NOT NULL,
  user_destination TEXT,
  admin_destination TEXT NOT NULL,
  token_program TEXT NOT NULL,
);

-- point refund_to @ token program
CREATE TABLE step(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program TEXT NOT  NULL,
  pool_program TEXT NOT  NULL,
  pool_authority TEXT NOT NULL,
  user_authority TEXT,
  user_source TEXT,
  pool_source TEXT NOT NULL,
  pool_destination TEXT NOT NULL,
  user_destination TEXT,
  pool_mint TEXT NOT NULL,
  pool_fee TEXT NOT NULL,
  refund_to TEXT NOT NULL,
  token_program TEXT NOT NULL,
);

CREATE TABLE stepn(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  parent_program TEXT NOT  NULL,
  pool_program TEXT NOT  NULL,
  pool_authority TEXT NOT NULL,
  user_authority TEXT,
  user_source TEXT,
  pool_source TEXT NOT NULL,
  pool_destination TEXT NOT NULL,
  user_destination TEXT,
  pool_mint TEXT NOT NULL,
  pool_fee TEXT NOT NULL,
  refund_to TEXT NOT NULL,
);

CREATE TABLE aldrin(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  pool_program TEXT NOT  NULL,
  pool_authority TEXT NOT NULL,
  pool_mint TEXT NOT NULL,
  pool_base_token_vault TEXT NOT NULL,
  pool_quote_token_vault TEXT NOT NULL,
  user_authority TEXT,
  user_base_token_account TEXT,
  user_quote_token_account TEXT,
  curve TEXT NOT NULL,
  pool_fee TEXT NOT NULL,
);

-- These keys need reviewing. `rent` points to sysvar rent
CREATE TABLE serum(
  id uuid NOT NULL,
  PRIMARY KEY (id),
  market TEXT NOT NULL,
  market_open_orders TEXT NOT NULL,
  market_request_queue TEXT NOT NULL,
  market_event_queue TEXT NOT NULL,
  market_bids TEXT NOT NULL,
  market_asks TEXT NOT NULL,
  market_order_payer_token_account TEXT NOT NULL,
  market_coin_vault TEXT NOT NULL,
  market_pc_vault TEXT NOT NULL,
  market_vault_signer TEXT NOT NULL,
  market_coin_wallet TEXT NOT NULL,
  user_authority TEXT,
  user_pc_wallet TEX,
  dex_program TEXT NOT NULL,
  token_program TEXT NOT NULL,
  rent TEXT NOT NULL,
);


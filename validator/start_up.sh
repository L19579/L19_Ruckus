#!/bin/bash

solana-validator \
--identity validator-keypair.json \
--vote-account vote-account-keypair.json \
--expected-genesis-hash 5eykt4UsFv8P8NJdTREpY1vzqKqZKvdpKuc147dw2N9d \
--known-validator 7Np41oeYqPefeNQEHSv1UDhYrehxin3NStELsSKCT4K2 \
--known-validator GdnSyH3YtwcxFvQrVVJMm1JhTS4QVX7MFsX56uJLUfiZ \
--known-validator DE1bawNcRJB9rVm3buyMVfr8mBEoyyu73NBovf2oXJsJ \
--known-validator CakcnaRDHka2gXyfbEd2d3xsvkJkqsLw2akB3zsN1D2S \
--entrypoint entrypoint.mainnet-beta.solana.com:8001 \
--entrypoint entrypoint2.mainnet-beta.solana.com:8001 \
--entrypoint entrypoint3.mainnet-beta.solana.com:8001 \
--entrypoint entrypoint4.mainnet-beta.solana.com:8001 \
--entrypoint entrypoint5.mainnet-beta.solana.com:8001 \
--accounts data/accounts \
--ledger data/ledger \
--log data/validator.log \
--no-voting \
--no-port-check \
--private-rpc \
--rpc-port 8899 \
--rpc-send-leader-count 4 \
--maximum-local-snapshot-age 2 \
--rpc-send-retry-ms 200 \
--dynamic-port-range 8000-8020 \
--limit-ledger-size 1000000000 \
--minimal-snapshot-download-speed 20971520 \
--wal-recovery-mode skip_any_corrupted_record \
--no-os-cpu-stats-reporting \
--no-os-memory-stats-reporting \
--no-os-network-stats-reporting

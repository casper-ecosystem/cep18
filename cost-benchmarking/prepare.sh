#!/bin/bash

rm cep18-cost-benchmarking-output

# NCTL config
CHAIN_NAME=casper-net-1
NODE_1_RPC_PORT=11101
NODE_1_ADDRESS=http://localhost:$NODE_1_RPC_PORT
USER_1_SECRET_KEY=$NCTL/assets/net-1/users/user-1/secret_key.pem
GAS_LIMIT=1000000000000

# Token installation args
TOKEN_WASM=target/wasm32-unknown-unknown/release/cep18.wasm
TOKEN_NAME="TestToken"
TOKEN_SYMBOL="TST"
TOKEN_DECIMALS=0
TOKEN_SUPPLY=10000000000

# Make sure our token wasm exists
cargo build --release --target wasm32-unknown-unknown -p cep18

# Install the token
TOKEN_INSTALL_DEPLOY=$(casper-client put-transaction\
  --chain-name $CHAIN_NAME\
  --node-address $NODE_1_ADDRESS\
  --secret-key $USER_1_SECRET_KEY\
  --payment-amount $GAS_LIMIT\
  --session-path $TOKEN_WASM\
  --session-arg "name:string='$TOKEN_NAME'"\
  --session-arg "symbol:string='$TOKEN_SYMBOL'"\
  --session-arg "decimals:u8='$TOKEN_DECIMALS'"\
  --session-arg "total_supply:u256='$TOKEN_SUPPLY'"\
  | jq .result.transaction_hash\
  | tr -d '"')

sleep 90

# Recover contract hash
TOKEN_CONTRACT_HASH=$(nctl-view-user-account user=1\
  | tr -d "\n"\
  | grep -o  "{.*"\
  | jq '.stored_value.Account.named_keys[] | select(.name == "cep18_token_contract") | .key'\
  | tr -d '"')

# Recover install cost
INSTALL_COST=$(nctl-view-chain-transaction deploy=$TOKEN_INSTALL_DEPLOY\
                | jq .execution_results[0].result.Success.cost\
                | tr -d '"')

echo INSTALLATION, $INSTALL_COST >> cep18-cost-benchmarking-output


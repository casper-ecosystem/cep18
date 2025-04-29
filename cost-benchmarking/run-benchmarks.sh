#!/bin/bash

SMALL_AMOUNT=1
# BE CAREFUL TO MAKE SURE THIS IS SIGNIFICANTLY LESS THAN TOTAL SUPPLY
BIG_AMOUNT=1000000000
BIG_SMALLER_AMOUNT=999999999
USER_1_ACCOUNT=$(nctl-view-user-account user=1\
  | grep -Pom1 "(?<=account_hash\": \")account-hash-[0-9|a-z|A-Z]{64}")
USER_2_ACCOUNT=$(nctl-view-user-account user=2\
  | grep -Pom1 "(?<=account_hash\": \")account-hash-[0-9|a-z|A-Z]{64}")
USER_3_ACCOUNT=$(nctl-view-user-account user=3\
  | grep -Pom1 "(?<=account_hash\": \")account-hash-[0-9|a-z|A-Z]{64}")

# Transfers from user 1 to user 2

TOKEN_SMALL_TRANSFER_DEPLOY=$(casper-client put-transaction\
        --chain-name $CHAIN_NAME\
        --node-address $NODE_1_ADDRESS\
        --secret-key $USER_1_SECRET_KEY\
        --payment-amount $GAS_LIMIT\
        --session-hash $TOKEN_CONTRACT_HASH\
        --session-entry-point "transfer"\
        --session-arg "recipient:key='$USER_2_ACCOUNT'"\
        --session-arg "amount:u256='$SMALL_AMOUNT'"\
        | jq .result.transaction_hash\
        | tr -d '"')

TOKEN_BIG_TRANSFER_DEPLOY=$(casper-client put-transaction\
        --chain-name $CHAIN_NAME\
        --node-address $NODE_1_ADDRESS\
        --secret-key $USER_1_SECRET_KEY\
        --payment-amount $GAS_LIMIT\
        --session-hash $TOKEN_CONTRACT_HASH\
        --session-entry-point "transfer"\
        --session-arg "recipient:key='$USER_2_ACCOUNT'"\
        --session-arg "amount:u256='$BIG_AMOUNT'"\
        | jq .result.transaction_hash\
        | tr -d '"')

# Approval from user 1 for user 2

TOKEN_SMALL_APPROVE_DEPLOY=$(casper-client put-transaction\
        --chain-name $CHAIN_NAME\
        --node-address $NODE_1_ADDRESS\
        --secret-key $USER_1_SECRET_KEY\
        --payment-amount $GAS_LIMIT\
        --session-hash $TOKEN_CONTRACT_HASH\
        --session-entry-point "approve"\
        --session-arg "spender:key='$USER_2_ACCOUNT'"\
        --session-arg "amount:u256='$SMALL_AMOUNT'"\
        | jq .result.transaction_hash\
        | tr -d '"')

sleep 120

TOKEN_BIG_APPROVE_DEPLOY=$(casper-client put-transaction\
        --chain-name $CHAIN_NAME\
        --node-address $NODE_1_ADDRESS\
        --secret-key $USER_1_SECRET_KEY\
        --payment-amount $GAS_LIMIT\
        --session-hash $TOKEN_CONTRACT_HASH\
        --session-entry-point "approve"\
        --session-arg "spender:key='$USER_2_ACCOUNT'"\
        --session-arg "amount:u256='$BIG_AMOUNT'"\
        | jq .result.transaction_hash\
        | tr -d '"')

sleep 120

# Indirect transfer from user 1 to user 3

TOKEN_SMALL_TRANSFER_FROM_DEPLOY=$(casper-client put-transaction\
        --chain-name $CHAIN_NAME\
        --node-address $NODE_1_ADDRESS\
        --secret-key $USER_2_SECRET_KEY\
        --payment-amount $GAS_LIMIT\
        --session-hash $TOKEN_CONTRACT_HASH\
        --session-entry-point "transfer_from"\
        --session-arg "owner:key='$USER_1_ACCOUNT'"\
        --session-arg "recipient:key='$USER_3_ACCOUNT'"\
        --session-arg "amount:u256='$SMALL_AMOUNT'"\
        | jq .result.transaction_hash\
        | tr -d '"')

TOKEN_BIG_TRANSFER_FROM_DEPLOY=$(casper-client put-transaction\
        --chain-name $CHAIN_NAME\
        --node-address $NODE_1_ADDRESS\
        --secret-key $USER_2_SECRET_KEY\
        --payment-amount $GAS_LIMIT\
        --session-hash $TOKEN_CONTRACT_HASH\
        --session-entry-point "transfer_from"\
        --session-arg "owner:key='$USER_1_ACCOUNT'"\
        --session-arg "recipient:key='$USER_3_ACCOUNT'"\
        --session-arg "amount:u256='$BIG_SMALLER_AMOUNT'"\
        | jq .result.transaction_hash\
        | tr -d '"')

sleep 120

# Write the data
DEPLOY_TYPES=(SMALL_TRANSFER BIG_TRANSFER SMALL_APPROVE BIG_APPROVE SMALL_TRANSFER_FROM BIG_TRANSFER_FROM)
for deploy_type in ${DEPLOY_TYPES[@]}; do
  name=\$TOKEN_$deploy_type\_DEPLOY
  cost=$(nctl-view-chain-deploy deploy=$(eval "echo $name")\
          | jq .execution_results[0].result.Success.cost\
          | tr -d '"')

  echo $deploy_type, $cost >> cep18-cost-benchmarking-output
done

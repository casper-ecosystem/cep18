#! /bin/bash
ARGS='[{"name": "method", "value": {"string_value": "deploy"}}, 
{"name": "amount", "value": {"big_int": {"value": "1", "bit_width": 512}}}]'

casperlabs_client --host deploy.casperlabs.io deploy \
    --public-key keys/validator-public.pem \
    --private-key keys/validator-private.pem \
    --session wasm/contract.wasm \
    --session-args "$ARGS" \
    --payment-amount 10000000
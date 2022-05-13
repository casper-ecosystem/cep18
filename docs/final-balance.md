
# Checking Final Balances

In this section, we check the final balance for all users.

**Final check for user A's balance**

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "erc20_test_call" \
--session-entry-point "check_balance_of" \
--session-arg "token_contract:account_hash='account-hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180'" \
--session-arg "address:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
--chain-name integration-test \
--payment-amount 1000000000
```

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash c1fb674c4b912a4562b146c8993576c773204dd5b1c731faf62b26386e34a373
```
<img src="/images/finalBalanceCheckAllUsers.png" alt="finalBalanceCheckAllUsers" title="Final Balance for All Users">
<br><br/>

**Final check for user B's balance**

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "erc20_test_call" \
--session-entry-point "check_balance_of" \
--session-arg "token_contract:account_hash='account-hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180'" \
--session-arg "address:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--chain-name integration-test \
--payment-amount 1000000000
```

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash 14f20fb57b7a600aed100a51a34deb6bfca3df1a03b31986a55d9f704ec48701
```
<img src="/images/finalBalanceCheckuserB.png" alt="finalBalanceCheckuserB" title="Final Balance for User B">
<br><br/>

**Final check for user C's balance**

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "erc20_test_call" \
--session-entry-point "check_balance_of" \
--session-arg "token_contract:account_hash='account-hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180'" \
--session-arg "address:key='account-hash-89422a0f291a83496e644cf02d2e3f9d6cbc5f7c877b6ba9f4ddfab8a84c2670'" \
--chain-name integration-test \
--payment-amount 1000000000
```

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash 72e523529f31ff13ea3a9463821b6981cbaf27c11d4a0f70e9b81127bb12e0c7
```
<img src="/images/finalBalanceCheckuserC.png" alt="finalBalanceCheckuserC" title="Final Balance for User C">
<br><br/>

**Final check of user D's balance**

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "erc20_test_call" \
--session-entry-point "check_balance_of" \
--session-arg "token_contract:account_hash='account-hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180'" \
--session-arg "address:key='account-hash-f32a2abc55316dc85a446a1c548674e03757974aaaf86e8b7d29947ae148eeca'" \
--chain-name integration-test \
--payment-amount 1000000000
```

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash 40c87ca9a4a78a37503ec87b5bfd9946267960135b1df0bb114403c18da4057d
```
<img src="/images/finalBalanceCheckuserD.png" alt="finalBalanceCheckuserD" title="Final Balance for User D">
<br><br/>

This marks the end of the sample guide documentation for the Casper fungible token.

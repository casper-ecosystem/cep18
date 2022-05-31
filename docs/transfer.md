# Token Transfers

## Transfer 50 Fungible Tokens from A to B

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--chain-name integration-test \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "transfer" \
--session-arg "recipient:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b" \
--session-arg "amount:u256='50'" \
--payment-amount "10000000000"
```

### Invoking `balance_of` Entry Point

We then call our utility contract to invoke the `balance_of` entry point and write the balance of User A to URef we sourced initially.

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

The follow up read of the corresponding URef:

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash eec441ad782dcaab1b83708a9456250f97e6725528f9579ca4871a3b9429033f
```
<img src="/images/transferUref.png" alt="transferUref" title="Transfer URef">
<br><br/>

### Transfer of 20 Fungible Tokens from B to C

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--chain-name integration-test \
--secret-key ~/casper/demo/user_b/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "transfer" \
--session-arg "recipient:key='account-hash-89422a0f291a83496e644cf02d2e3f9d6cbc5f7c877b6ba9f4ddfab8a84c2670'" \
--session-arg "amount:u256='20'" \
--payment-amount "10000000000"
```

We must then call the utility contract once for every balance we wish to check.

**Invoking `balance_of` for user A**

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

```
casper-client query-global-state -n http://3.143.158.19:7777
--state-root-hash eec441ad782dcaab1b83708a9456250f97e6725528f9579ca4871a3b9429033f \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
```
<img src="/images/invoke-bal-a.png" alt="invoke-bal-a" title="Invoking balance_of for User A">
<br><br/>

**Invoking `balance_of` for user B**

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
--state-root-hash 3e77ef8615f372d8d169959c9ebd276693ec98e7d69b62e3872ffe4328e6427c \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007
```
<img src="/images/invoke-bal-b.png" alt="invoke-bal-b" title="Invoking balance_of for User B">
<br><br/>

**Invoking `balance_of` for user C**

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
--state-root-hash 745aa27b61cf37dd1f3d0f57212874a9430ea4fa597c54d25a02ba5f4665ca37 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007
```
<img src="/images/invoke-bal-c.png" alt="invoke-bal-c" title="Invoking balance_of for User C">
<br><br/>

### Approve B to Spend 15 Tokens of A

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--chain-name integration-test \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "approve" \
--session-arg "spender:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "amount:u256='15'" \
--payment-amount "10000000000"
```

**Invoking `allowance` entry point to check for the allowance**

As we did for `balance_of` to read out the balance values, we must perform a similar invoking of the `allowance` entry point of the main fungible token contract.

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "erc20_test_call" \
--session-entry-point "check_allowance_of" \
--session-arg "token_contract:account_hash='account-hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180'" \
--session-arg "owner:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
--session-arg "spender:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--chain-name integration-test \
--payment-amount 10000000000
```

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--state-root-hash a4f11712b6ffe2f87267d2fa08aa04494846d50ed8a3d1717724a1d6facfe2a7 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007
```
<img src="/images/invoke-bal-a.png" alt="invoke-bal-a" title="Invoking balance_of for User A">
<br><br/>

### Transfer 10 Fungible Tokens from B’s Allowance to D

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--chain-name integration-test \
--secret-key ~/casper/demo/user_b/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "transfer_from" \
--session-arg "owner:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
--session-arg "recipient:key='account-hash-f32a2abc55316dc85a446a1c548674e03757974aaaf86e8b7d29947ae148eeca'" \
--session-arg "amount:u256='10'" \
--payment-amount "10000000000"
```

**Invoking `balance_of` for user A**

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
--state-root-hash 76b1e6844b26d0565461e8d609147ea5c0e0f6f70022d2a9ebfbabce6f5f8407 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007
```
<img src="/images/invoke-bal-a.png" alt="invoke-bal-a" title="Invoking balance_of for User A">
<br><br/>

**Invoking `balance_of` for user B**

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
--state-root-hash 08f89451f65d87aac27f482bfb46d6772ee7543c207fd2907a2515549ed01a9a \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007
```
<img src="/images/invoke-bal-b2.png" alt="invoke-bal-b2" title="Invoking balance_of for User B">
<br><br/>

**Invoking `balance_of` for user C**

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
--state-root-hash 717c4b166737cec5a3101ecea2341f96e54dde17309ecfbe80a3df339d8e4bcd \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007
```
<img src="/images/invoke-bal-c2.png" alt="invoke-bal-c2" title="Invoking balance_of for User C">
<br><br/>

**Invoking `balance_of` for user D**

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
<img src="/images/invoke-bal-d.png" alt="invoke-bal-d" title="Invoking balance_of for User D">
<br><br/>

**Invoking `allowance` to check that it is 5**

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "erc20_test_call" \
--session-entry-point "check_allowance_of" \
--session-arg "token_contract:account_hash='account-hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180'" \
--session-arg "owner:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
--session-arg "spender:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--chain-name integration-test \
--payment-amount 10000000000
```

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--state-root-hash 90635f6e9c35df061e74903148a1b47b9f32c1feb40abb7a902163a20f4c2025 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007
```
<img src="/images/invoke-allowance.png" alt="invoke-allowance" title="Invoking Allowance for Verification">
<br><br/>

Refer to the [token approval](approve.md) guide to follow the next steps.



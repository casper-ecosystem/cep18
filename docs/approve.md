
# Token Approvals

### Transfer 10 Tokens from an Allowance of only 5

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

Since we know that the allowance value is less than 10, we expect the deploy to fail.

**_Note:_**

> Here is an example of a [deploy failure due to overspending an allowance](https://integration.cspr.live/deploy/7a692917b91e1485f500966f3884bb0917006725505fec1ce3aed2a13ec692df)

### Additional transfer_from of Remainder 5 tokens

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--chain-name integration-test \
--secret-key ~/casper/demo/user_b/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "transfer_from" \
--session-arg "owner:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
--session-arg "recipient:key='account-hash-f32a2abc55316dc85a446a1c548674e03757974aaaf86e8b7d29947ae148eeca'" \
--session-arg "amount:u256='5'" \
--payment-amount "10000000000"
```

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

```bash

casper-client query-global-state -n http://3.143.158.19:7777 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash d068439dc1f62e330a15e008e5e926e777fd3599baed4ece508d482c50bd263b
```
<img src="/images//invokeBalanceOfuserD.png" alt="/invokeBalanceOfuserD" title="Invoking balance_of for User D">
<br><br/>


**Invoking `allowance` for B’s tokens of A**

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "erc20_test_call" \
--session-entry-point "check_allowance_of" \
--session-arg "token_contract:account_hash='account-hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180'" \
--session-arg "owner:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
--session-arg "spender:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'"
```

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash e863633b47b0689033744855739009b45a8654dadd4ed723f527fd38157a1d92
```
<img src="/images/invokeAllowanceBsTokenforA.png" alt="invokeAllowanceBsTokenforA" title="Invoking allowance for Bs tokens of A">
<br><br/>


### Approving C to spend 10 of B’s Fungible Tokens

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--chain-name integration-test \
--secret-key ~/casper/demo/user_b/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "approve" \
--session-arg "spender:key='account-hash-89422a0f291a83496e644cf02d2e3f9d6cbc5f7c877b6ba9f4ddfab8a84c2670'" \
--session-arg "amount:u256='10'" \
--payment-amount "10000000000"
```

**Invoking `allowance` to check C’s allowance**

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "erc20_test_call" \
--session-entry-point "check_allowance_of" \
--session-arg "token_contract:account_hash='account-hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180'" \
--session-arg "owner:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "spender:key='account-hash-89422a0f291a83496e644cf02d2e3f9d6cbc5f7c877b6ba9f4ddfab8a84c2670'" \
--chain-name integration-test \
--payment-amount 10000000000
```

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash e9f069c2c03b18f86c15fec54286ac66ece368ac36d9d28024d0cd6cfc93fcf5
```
<img src="/images/invokingToCheckCsAllowance.png" alt="invokingToCheckCsAllowance" title="Check allowance of C">
<br><br/>


## Transfer_from C’s Allowance to D

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--chain-name integration-test \
--secret-key ~/casper/demo/user_c/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "transfer_from" \
--session-arg "owner:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "recipient:key='account-hash-f32a2abc55316dc85a446a1c548674e03757974aaaf86e8b7d29947ae148eeca'" \
--session-arg "amount:u256='5'" \
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
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash eb506808fe0749364163fea646c3f4ef35bb55363ea849da219badcd6ba3ee80
```
<img src="/images/invokingBalanceOfuserA.png" alt="invokingBalanceOfuserA" title="Invoking balance_of for User A">
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
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash 0ce2c4991543758337a38d1d8f7fe56a42616b95ec93b17aec35a6f03b5e389c
```
<img src="/images/invokingBalanceOfuserB.png" alt="invokingBalanceOfuserB" title="Invoking balance_of for User B">
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
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash 215c50c0e86950cb91bd8e1045315c1129bbaa02d4e49e00bed60130c4dfa69c
```
<img src="/images/invokingBalanceOfuserC1.png" alt="invokingBalanceOfuserC1" title="Invoking balance_of for User C">
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

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash 4e8b0de303f834cb7c61bef148046e3de4446903bd15a395c9c37a6d96efe8c6
```
<img src="/images/invokingBalanceOfuserD.png" alt="invokingBalanceOfuserD" title="Invoking balance_of for User D">
<br><br/>

**Invoking `allowance` to check C’s allowance**

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "erc20_test_call" \
--session-entry-point "check_allowance_of" \
--session-arg "token_contract:account_hash='account-hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180'" \
--session-arg "owner:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "spender:key='account-hash-89422a0f291a83496e644cf02d2e3f9d6cbc5f7c877b6ba9f4ddfab8a84c2670'" \
--chain-name integration-test \
--payment-amount 10000000000
```

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash d6d4d3e59017dfc21e2c9a8e235e2a2b3a446284a066a1f1f6704559fbb35a66
```
<img src="/images/invokingAlToCheckCsAllowance.png" alt="invokingAlToCheckCsAllowance" title="Check Allowance for C">
<br><br/>

### Failure to Overspend C's Allowance\*\*

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--chain-name integration-test
--secret-key ~/casper/demo/user_c/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "transfer_from" \
--session-arg "owner:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "recipient:key='account-hash-f32a2abc55316dc85a446a1c548674e03757974aaaf86e8b7d29947ae148eeca'" \
--session-arg "amount:u256='10'" \
--payment-amount "10000000000"
```

**_Note:_**

> Here is an example of a [failure to overspend C's allowance](https://integration.cspr.live/deploy/db50ac05fe63561669b9d73c28b66fcb5a341048d5d13b1b2759b557396fd5d2)

**Invoking `allowance` to check C’s allowance**

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "erc20_test_call" \
--session-entry-point "check_allowance_of" \
--session-arg "token_contract:account_hash='account-hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180'" \
--session-arg "owner:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b'" \
--session-arg "spender:key='account-hash-89422a0f291a83496e644cf02d2e3f9d6cbc5f7c877b6ba9f4ddfab8a84c2670'" \
--chain-name integration-test \
--payment-amount 10000000000
```

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--key uref-56efe683287668bab985d472b877b018ad24a960aafadb48ebc5217737b45c85-007 \
--state-root-hash be29754920f158f093c1daac780fba37bed06c751f256a43fcdc7b5b2775e487
```
<img src="/images/invokingToCheckCsAllowance3.png" alt="invokingToCheckCsAllowance3" title="Invoking to Check Cs Allowance">
<br><br/>

Refer to the [check final balance](final-balance.md) guide to follow the next steps.


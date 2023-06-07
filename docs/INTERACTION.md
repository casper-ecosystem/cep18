# CEP-18 Contract Interaction Tutorial

This tutorial explains the various ways you may interact with your Casper fungible token contract and includes the following subjects:

- [Querying the Contract Package](#querying-the-contract-package)
- [CEP-18 Token Transfers](#cep-18-token-transfers)
- [Token Allowance Approvals](#token-allowance-approvals)
- [Checking the Final Balances](#checking-the-final-balances)

## Querying the Contract Package

We will need the contract package's `contract_hash` to interact with the recently installed instance of CEP-18. You can find the contract package hash within the installing account's `NamedKeys`, under the name given during the installation process. In this instance, you will be looking for `

```bash
casper-client query-global-state -n http://<HOST IP>:<PORT> \
// This is the contract package hash, which can be found within the `NamedKeys` of the account that sent the installing deploy.
--key hash-82bd86d2675b2dc44c19027fb7717a99db6fda5e0cad8d597f2495a9dbc9df7f \
// This is the most up to date state root hash, which can found by using the `get-state-root-hash` command in the Casper client.
--state-root-hash f9f73c3a4da5893b67c4cac94a5695d76cfefff61b050c98a7b19e2b8efd3933
```

This will return the `Contract Package` object:

```bash
{
  "id": -1489823435760214673,
  "jsonrpc": "2.0",
  "result": {
    "api_version": "1.0.0",
    "block_header": null,
    "merkle_proof": "[2048 hex chars]",
    "stored_value": {
      "ContractPackage": {
        "access_key": "uref-8dac847ce0ae20f0156cf37dd233cc1d166fde8269fc9a393b0ea04174be1167-007",
        "disabled_versions": [],
        "groups": [],
        "versions": [
          {
            "contract_hash": "contract-05d893e76c731729fc26339e5a970bd79fbf4a6adf743c8385431fb494bff45e",
            "contract_version": 1,
            "protocol_version_major": 1
          }
        ]
      }
    }
  }
}
```

* Note - In the `contract_hash` field, the hash value represents the stored contract which we will invoke later.

In addition, there is a utility contract that invokes the various balance and allowance entry points of the main fungible token contract. Upon receiving the returned value, the utility contract will write the value to a URef called `result`. You can find this URef in the `NamedKeys` of the utility contract.

First, you will need to query the `cep18_test_contract` hash found within the installing account's `NamedKeys`:

```bash
casper-client query-global-state -n http://<HOST IP>:<PORT> \
// This is the contract hash for the `cep18_test_contract` as found from the installing account's `NamedKeys`
--key hash-015b99020edb40e7e1e2b31a8e104bc226242f960a2d10dc1d91ae3eb6fa41b6 \
--state-root-hash f9f73c3a4da5893b67c4cac94a5695d76cfefff61b050c98a7b19e2b8efd3933
```

Which should return information similar to the following:

```bash

{
  "id": 5359405942597097786,
  "jsonrpc": "2.0",
  "result": {
    "api_version": "1.0.0",
    "block_header": null,
    "merkle_proof": "[2048 hex chars]",
    "stored_value": {
      "ContractPackage": {
        "access_key": "uref-1b867a3751f505762c69c8d92ba7462818cd0c2a705bb5d4270bce479410ee55-007",
        "disabled_versions": [],
        "groups": [],
        "versions": [
          {
            "contract_hash": "contract-a8fe057675930f0951d45816c55615228ac8af2b7b231788278dffcf1dd8c0ca",
            "contract_version": 1,
            "protocol_version_major": 1
          }
        ]
      }
    }
  }
}

```

You will need to take the `contract_hash` value and replace `contract` with `hash` to run another `query-global-state:

```bash
casper-client query-global-state -n http://<HOST IP>:<PORT> \
--key hash-a8fe057675930f0951d45816c55615228ac8af2b7b231788278dffcf1dd8c0ca \
--state-root-hash f9f73c3a4da5893b67c4cac94a5695d76cfefff61b050c98a7b19e2b8efd3933
```

Which will return the full `cep18_test_contract` information. The following snippet is condensed to show only the `NamedKeys`, but you should also see the `entry_points` when you run the command. You should see the URef `result`, which will be used to view the results of any checks run through the utility contract.

```bash
{
  "id": -1426549275795832481,
  "jsonrpc": "2.0",
  "result": {
    "api_version": "1.0.0",
    "block_header": null,
    "merkle_proof": "[3370 hex chars]",
    "stored_value": {
      "Contract": {
        "contract_package_hash": "contract-package-015b99020edb40e7e1e2b31a8e104bc226242f960a2d10dc1d91ae3eb6fa41b6",
        "contract_wasm_hash": "contract-wasm-7959083a4df983ddcd3a9ae46af092dbf126031181ab2619ddc64db09bde8c27",
        "named_keys": [
          {
            "key": "uref-a46ad389b53715d9991a513c8ca48e1502facc4c563c0700a31e830c4cb8a7d4-007",
            "name": "result"
          }
        ],
        "protocol_version": "1.0.0"
      }
    }
  }
}

```

## CEP-18 Token Transfers

### Transfer 50 Fungible Tokens from A to B

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

#### Invoking `balance_of` Entry Point

We then call our utility contract to invoke the `balance_of` entry point and write the balance of User A to URef we sourced initially.

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_test_call" \
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

#### Transfer of 20 Fungible Tokens from B to C

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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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

#### Approve B to Spend 15 Tokens of A

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
--session-package-name "cep18_test_call" \
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

#### Transfer 10 Fungible Tokens from B’s Allowance to D

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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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

## Token Allowance Approvals

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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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


### Transfer_from C’s Allowance to D

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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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

### Failure to Overspend C's Allowance

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
--session-package-name "cep18_test_call" \
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

## Checking the Final Balances

In this section, we check the final balance for all users.

**Final check for user A's balance**

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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
--session-package-name "cep18_test_call" \
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

# CEP-18 Token Transfers and Allowances

This document describes how to transfer CEP-18 tokens on a Casper network using the Casper client. The [Exploring the CEP18 Contracts](./2-query.md) documentation contains more in depth explanations on how to find the various hashes and URefs referenced throughout this document.

## Transferring CEP-18 Tokens to Another Account

The following command will invoke the `transfer` entry point on your instance of CEP-18, directing it to transfer 10 of the associated CEP-18 tokens to another account.

```bash
casper-client put-transaction -n http://<node IP>:<PORT> \
// The chain name of the Casper network on which your CEP-18 instance was installed.
--chain-name <CHAIN NAME>\
// The local path to your account's secret key.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// The contract hash of your CEP-18 contract instance.
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
// The name of the entry point you are invoking.
--session-entry-point "transfer" \
// The account hash of the account that you are sending CEP-18 tokens to.
--session-arg "recipient:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b" \
// The amount of CEP-18 tokens you are sending to the receiving account.
--session-arg "amount:u256='10'" \
// The gas payment you are allotting, in motes.
--payment-amount "10000000000"
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client put-transaction -n http://<node IP>:<PORT> \
--chain-name <CHAIN NAME>\
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180 \
--session-entry-point "transfer" \
--session-arg "recipient:key='account-hash-9f81014b9c7406c531ebf0477132283f4eb59143d7903a2fae54358b26cea44b" \
--session-arg "amount:u256='50'" \
--payment-amount "10000000000"
```

</details>

This command will return a deploy hash that you can query using `casper-client get-deploy`. Querying the Deploy allows you to verify execution success, but you will need to use the `check_balance_of` entry point on the utility contract to verify the account's balance.

### Invoking the `check_balance_of` Entry Point

The following Casper client command invokes the `check_balance_of` entry point on the `cep18_test_contract`.

```bash
casper-client put-transaction -n http://<node IP>:<PORT>\
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_test_contract" \
--session-entry-point "check_balance_of" \
// This is the contract hash of your CEP-18 contract instance, passed in as an `account-hash-`.
--session-arg "token_contract:account_hash='account-hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180'" \
// This is the account hash of the account you are checking the balance of.
--session-arg "address:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
--chain-name <CHAIN NAME> \
--payment-amount 1000000000
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client put-transaction -n http://<node IP>:<PORT>\
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_test_contract" \
--session-entry-point "check_balance_of" \
--session-arg "token_contract:account_hash='account-hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180'" \
--session-arg "address:key='account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd'" \
--chain-name <CHAIN NAME> \
--payment-amount 1000000000
```

</details>

After sending this command, you will need to query the `results` URef within the `NamedKeys` of your `cep18_test_contract` utility contract instance. More information on finding this URef can be found in the [Exploring the CEP18 Contracts](./query.md#querying-the-utility-contract) document.

You can use the following command to query global state for the `results` URef.

```bash
casper-client query-global-state -n http://<NODE IP>:<PORT> \
// This is the `results` URef location from your `cep18_test_contract` `NamedKeys`
--key uref-a46ad389b53715d9991a513c8ca48e1502facc4c563c0700a31e830c4cb8a7d4-007 \
--state-root-hash 3aecd0e4b6ec29ee7c1eed701132eabfe6e66a1e0f1595c9c65bfed447e474f7
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client query-global-state -n http://<NODE IP>:<PORT> \
--key uref-a46ad389b53715d9991a513c8ca48e1502facc4c563c0700a31e830c4cb8a7d4-007 \
--state-root-hash 3aecd0e4b6ec29ee7c1eed701132eabfe6e66a1e0f1595c9c65bfed447e474f7
```

</details>

This command should show something similar to the following in response, with `parsed` being the amount of CEP-18 tokens that the account holds.

```bash
{
  "id": -8841145064950441692,
  "jsonrpc": "2.0",
  "result": {
    "api_version": "1.0.0",
    "block_header": null,
    "merkle_proof": "[3796 hex chars]",
    "stored_value": {
      "CLValue": {
        "bytes": "010a",
        "cl_type": "U256",
        "parsed": "10"
      }
    }
  }
}
```

## Approving an Allowance for Another Account

The Casper fungible token contract features an `allowance` entry point that allows an account to delegate another account to spend a preset number of CEP-18 tokens from their balance.

### Approving an Account to Spend Tokens on Another Account's Behalf

The following command approves a third-party account to spend an `allowance` of 15 CEP-18 tokens from the balance of the account that sent the CEP-18 instance.

```bash
casper-client put-transaction -n http://<node IP>:<PORT>\
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
// This is the contract hash of the CEP-18 token contract.
--session-hash hash-05d893e76c731729fc26339e5a970bd79fbf4a6adf743c8385431fb494bff45e \
--session-entry-point "approve" \
// This is the account hash of the account that will receive an allowance from the balance of the account that sent the Deploy.
--session-arg "spender:key='account-hash-17192017d32db5dc9f598bf8ac6ac35ee4b64748669b00572d88335941479513'" \
// This is the number of CEP-18 tokens included in the allowance.
--session-arg "amount:u256='15'" \
--payment-amount "10000000000"
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client put-transaction -n http://<node IP>:<PORT>\
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-05d893e76c731729fc26339e5a970bd79fbf4a6adf743c8385431fb494bff45e \
--session-entry-point "approve" \
--session-arg "spender:key='account-hash-17192017d32db5dc9f598bf8ac6ac35ee4b64748669b00572d88335941479513'" \
--session-arg "amount:u256='15'" \
--payment-amount "10000000000"
```

</details>

### Verifying a Previously Issued Allowance

After approving an account to spend an `allowance` of tokens, we can verify the allotted allowance by using the utility contract. The following command will write the `allowance` of the spender's account to the `result` URef of in the utility contract's `NamedKeys`:

```bash
casper-client put-transaction -n http://<node IP>:<PORT>\
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_test_contract" \
--session-entry-point "check_allowance_of" \
// This is the contract hash for the CEP-18 token.
--session-arg "token_contract:account_hash='account-hash-05d893e76c731729fc26339e5a970bd79fbf4a6adf743c8385431fb494bff45e'" \
// This is the account hash for the account that owns the CEP-18 tokens.
--session-arg "owner:key='account-hash-39f15c23df9be1244572bb499fac62cbcad3cab2dc1438609842f602f943d7d2'" \
// This is the account hash for the account previously authorized to spend an allowance of the owning account's CEP-18 tokens.
--session-arg "spender:key='account-hash-17192017d32db5dc9f598bf8ac6ac35ee4b64748669b00572d88335941479513'" \
--chain-name <CHAIN NAME> \
--payment-amount 10000000000
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client put-transaction -n http://<node IP>:<PORT>\
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_test_contract" \
--session-entry-point "check_allowance_of" \
--session-arg "token_contract:account_hash='account-hash-05d893e76c731729fc26339e5a970bd79fbf4a6adf743c8385431fb494bff45e'" \
--session-arg "owner:key='account-hash-39f15c23df9be1244572bb499fac62cbcad3cab2dc1438609842f602f943d7d2'" \
--session-arg "spender:key='account-hash-17192017d32db5dc9f598bf8ac6ac35ee4b64748669b00572d88335941479513'" \
--chain-name <CHAIN NAME> \
--payment-amount 10000000000
```

</details>

The following command queries global state to return the value stored under the `result` URef:

```bash
casper-client query-global-state -n http://<node IP>:<PORT> \
// This is the previously identified `result` URef from the utility contract's `NamedKeys`
--key uref-a46ad389b53715d9991a513c8ca48e1502facc4c563c0700a31e830c4cb8a7d4-007 \
--state-root-hash e64f877f65df26db74300bb175c244d589bd88a23b91abf9ceb73ac5e65e90f1
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client query-global-state -n http://<node IP>:<PORT> \
--key uref-a46ad389b53715d9991a513c8ca48e1502facc4c563c0700a31e830c4cb8a7d4-007 \
--state-root-hash e64f877f65df26db74300bb175c244d589bd88a23b91abf9ceb73ac5e65e90f1
```

</details>

You should get a response similar to the following:

```bash
{
  "id": -9142472925449984061,
  "jsonrpc": "2.0",
  "result": {
    "api_version": "1.0.0",
    "block_header": null,
    "merkle_proof": "[3796 hex chars]",
    "stored_value": {
      "CLValue": {
        "bytes": "010f",
        "cl_type": "U256",
        "parsed": "15"
      }
    }
  }
}
```

### Transferring Tokens from an Allowance

The following command allows an account to transfer CEP-18 tokens held by another account up to their approved `allowance`.

```bash
casper-client put-transaction -n http://<NODE IP>:<PORT> \
--chain-name <CHAIN NAME> \
// This is the secret key for the account that is spending their `allowance` from another account's balance.
--secret-key ~/casper/demo/user_a/secret_key.pem \
// This is the CEP-18 token contract.
--session-hash hash-05d893e76c731729fc26339e5a970bd79fbf4a6adf743c8385431fb494bff45e \
--session-entry-point "transfer_from" \
// This is the account hash of the account that holds the CEP-18 in their balance.
--session-arg "owner:key='account-hash-39f15c23df9be1244572bb499fac62cbcad3cab2dc1438609842f602f943d7d2'" \
// This is the account hash of the account that will receive the transferred CEP-18 tokens.
--session-arg "recipient:key='account-hash-17192017d32db5dc9f598bf8ac6ac35ee4b64748669b00572d88335941479513'" \
// This is the amount of tokens to be transferred. If this amount exceeds the `allowance` of the account sending the Deploy, it will fail.
--session-arg "amount:u256='10'" \
--payment-amount "10000000000"
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client put-transaction -n http://<NODE IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-hash hash-05d893e76c731729fc26339e5a970bd79fbf4a6adf743c8385431fb494bff45e \
--session-entry-point "transfer_from" \
--session-arg "owner:key='account-hash-39f15c23df9be1244572bb499fac62cbcad3cab2dc1438609842f602f943d7d2'" \
--session-arg "recipient:key='account-hash-17192017d32db5dc9f598bf8ac6ac35ee4b64748669b00572d88335941479513'" \
--session-arg "amount:u256='10'" \
--payment-amount "10000000000"
```

</details>

### Increasing and Decreasing an Allowance

#### Increasing an Allowance

The following command increases the designated `allowance` for the provided account.

```bash
casper-client put-transaction -n http://<NODE IP>:<PORT> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_contract_package_CEP18" \
--session-entry-point "increase_allowance" \
// This is the account hash of the previously authorized allowance account.
--session-arg "spender:key='account-hash-683f53f56926f54ef9584b07585b025c68415dc05f7b2e56749153574b83d5cd'" \
// This is the additional number of CEP-18 tokens that the authorized account may spend.
--session-arg "amount:U256='10'" \
--chain-name <CHAIN NAME> \
--payment-amount 1000000000
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client put-transaction -n http://<NODE IP>:<PORT> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_contract_package_CEP18" \
--session-entry-point "increase_allowance" \
--session-arg "spender:key='account-hash-683f53f56926f54ef9584b07585b025c68415dc05f7b2e56749153574b83d5cd'" \
--session-arg "amount:U256='10'" \
--chain-name <CHAIN NAME> \
--payment-amount 1000000000
```

</details>

#### Decreasing an Allowance

The following command decreases the designated allowance for the provided account.

```bash
casper-client put-transaction -n http://<NODE IP>:<PORT> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_contract_package_CEP18" \
--session-entry-point "decrease_allowance" \
// This is the account hash of the previously authorized allowance account.
--session-arg "spender:key='account-hash-683f53f56926f54ef9584b07585b025c68415dc05f7b2e56749153574b83d5cd'" \
// This is the additional number of CEP-18 tokens that the authorized account may spend.
--session-arg "amount:U256='10'" \
--chain-name <CHAIN NAME> \
--payment-amount 1000000000
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client put-transaction -n http://<NODE IP>:<PORT> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_contract_package_CEP18" \
--session-entry-point "decrease_allowance" \
--session-arg "spender:key='account-hash-683f53f56926f54ef9584b07585b025c68415dc05f7b2e56749153574b83d5cd'" \
--session-arg "amount:U256='10'" \
--chain-name <CHAIN NAME> \
--payment-amount 1000000000
```

</details>

### Minting and Burning Additional CEP-18 Tokens

#### Minting Additional Tokens

If the contract allows for minting, the following command will mint a number of CEP-18 tokens directly to the provided account. This increases the total supply of the token in question.

```bash
casper-client put-transaction -n http://<NODE IP>:<PORT> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_contract_package_CEP18" \
--session-entry-point "mint" \
// This is the account that will receive the newly minted CEP-18 tokens.
--session-arg "owner:key='account-hash-683f53f56926f54ef9584b07585b025c68415dc05f7b2e56749153574b83d5cd'" \
// This is the number of additional CEP-18 tokens to add to the total supply.
--session-arg "amount:U256='10'" \
--chain-name <CHAIN NAME> \
--payment-amount 1000000000
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client put-transaction -n http://<NODE IP>:<PORT> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_contract_package_CEP18" \
--session-entry-point "mint" \
--session-arg "owner:key='account-hash-683f53f56926f54ef9584b07585b025c68415dc05f7b2e56749153574b83d5cd'" \
--session-arg "amount:U256='10'" \
--chain-name <CHAIN NAME> \
--payment-amount 1000000000
```

</details>

#### Burning Tokens

If the contract allows for burning, the following command will burn a number of CEP-18 tokens directly from the provided account. This decreases the total supply of the token in question.

```bash
casper-client put-transaction -n http://<NODE IP>:<PORT> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_contract_package_CEP18" \
--session-entry-point "burn" \
// This is the account that the tokens will be burned from.
--session-arg "owner:key='account-hash-683f53f56926f54ef9584b07585b025c68415dc05f7b2e56749153574b83d5cd'" \
// This is the number of CEP-18 tokens to remove from the total supply.
--session-arg "amount:U256='10'" \
--chain-name <CHAIN NAME> \
--payment-amount 1000000000
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client put-transaction -n http://<NODE IP>:<PORT> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_contract_package_CEP18" \
--session-entry-point "burn" \
// This is the account that the tokens will be burned from.
--session-arg "owner:key='account-hash-683f53f56926f54ef9584b07585b025c68415dc05f7b2e56749153574b83d5cd'" \
// This is the number of CEP-18 tokens to remove from the total supply.
--session-arg "amount:U256='10'" \
--chain-name <CHAIN NAME> \
--payment-amount 1000000000
```

</details>

### Changing Account Security Permissions

The `change_security` entrypoint can be used by an account with `admin` access to alter the security level of other accounts as described [here](../cep18/README.md#changing-security-access).

There are five security levels, with the strongest level taking precedence over any other assigned levels. In order of highest strength to lowest:

1. `None` - `None` overrides other security levels and removes all admin, minting and burning access of an account.

2. `Admin` - Allows the account full access and control over the CEP-18 contract.

3. `MintAndBurn` - Allows the account to mint new tokens and burn existing tokens.

4. `Burner` - The account can burn tokens.

5. `Minter` - The account can mint new tokens.

Here is an example of a `session-arg` that provides a list of account hashes to be included on the `mint_and_burn_list`:

```bash
--session-arg "mint_and_burn_list:string='account-hash-1ed5a1c39bea93c105f2d22c965a84b205b36734a377d05dbb103b6bfaa595a7,account-hash-0ea7998b2822afe5b62b08a21d54c941ad791279b089f3f7ede0d72b477eca34,account-hash-e70dbca48c2d31bc2d754e51860ceaa8a1a49dc627b20320b0ecee1b6d9ce655'"
```

**Be aware that removing all admin accounts will lock out all admin functionality.**

The following command can be supplied with any of the optional arguments above:

```bash
casper-client put-transaction -n http://<NODE IP>:<PORT> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-package-name "cep18_contract_package_CEP18" \
--session-entry-point "change_security" \
/// The following arguments are all optional and each consists of a string of the account hashes to be added to the list specified, separated by commas.
--session-arg "none_list:string:'<List of account hashes>'" \
--session-arg "admin_list:string:'<List of account hashes>'" \
--session-arg "mint_and_burn_list:string:'<List of account hashes>'" \
--session-arg "burner_list:string:'<List of account hashes>'" \
--chain-name <CHAIN NAME> \
--payment-amount 1000000000
```

### Next Steps

- [Testing Framework for CEP-18](./4-tests.md)

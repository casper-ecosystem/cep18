# Exploring the CEP18 Contracts

This document covers the necessary information that you will need to interact with your CEP-18 contract instance. Your setup should include the following two contracts:

- The Casper fungible token contract

- The CEP-18 utility contract, which should appear in the `NamedKeys` of the account that sent the Deploy as `cep18_test_contract`

## Querying the Contract Package

We will need the contract package's `contract_hash` to interact with the recently installed instance of CEP-18. You can find the contract package hash within the installing account's `NamedKeys`, under the name given during the installation process.

```bash
casper-client query-global-state -n http://<HOST IP>:<PORT> \
// This is the contract package hash, which can be found within the `NamedKeys` of the account that sent the installing deploy.
--key hash-82bd86d2675b2dc44c19027fb7717a99db6fda5e0cad8d597f2495a9dbc9df7f \
// This is the most up to date state root hash, which can found by using the `get-state-root-hash` command in the Casper client.
--state-root-hash f9f73c3a4da5893b67c4cac94a5695d76cfefff61b050c98a7b19e2b8efd3933
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client query-global-state -n http://<HOST IP>:<PORT> \
--key hash-82bd86d2675b2dc44c19027fb7717a99db6fda5e0cad8d597f2495a9dbc9df7f \
--state-root-hash f9f73c3a4da5893b67c4cac94a5695d76cfefff61b050c98a7b19e2b8efd3933
```

</details>

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

## Querying the Utility Contract

In addition, there is a utility contract that invokes the various balance and allowance entry points of the main fungible token contract. Upon receiving the returned value, the utility contract will write the value to a URef called `result`. You can find this URef in the `NamedKeys` of the utility contract.

First, you will need to query the `cep18_test_contract` hash found within the installing account's `NamedKeys`:

```bash
casper-client query-global-state -n http://<HOST IP>:<PORT> \
// This is the contract hash for the `cep18_test_contract` as found from the installing account's `NamedKeys`
--key hash-015b99020edb40e7e1e2b31a8e104bc226242f960a2d10dc1d91ae3eb6fa41b6 \
--state-root-hash f9f73c3a4da5893b67c4cac94a5695d76cfefff61b050c98a7b19e2b8efd3933
```

<details>
<summary><b>Casper client command without comments</b></summary>

```bash
casper-client query-global-state -n http://<HOST IP>:<PORT> \
--key hash-015b99020edb40e7e1e2b31a8e104bc226242f960a2d10dc1d91ae3eb6fa41b6 \
--state-root-hash f9f73c3a4da5893b67c4cac94a5695d76cfefff61b050c98a7b19e2b8efd3933
```
</details>

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

## Next Steps

- [CEP-18 Token Transfers and Allowances](./3-transfer.md)
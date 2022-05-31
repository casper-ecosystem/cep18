# Query the Contract Package

Query the contract package hash which returns the package:

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--key hash-f9ae16d2a374e985c425c94c716c539070dc7d7907f119ba32d9684f4412a4ee \
--state-root-hash 4e8b0de303f834cb7c61bef148046e3de4446903bd15a395c9c37a6d96efe8c6
```

This will return the `Contract Package` object:

<img src="/images/contract-pkg.png" alt="contract-package" title="Contract Package Object">

**_Note:_**

> In the `contract_hash` field, the hash value represents the stored contract which we will invoke later on.

The utility contract invokes the `balance_of` and `allowance` entry point of the main fungible token contract. Once the value is returned, the utility contract will write either a balance or an allowance value to a URef within its named keys, called `result`. We will query the utility contract object to read the URef.

```bash
casper-client query-global-state -n http://3.143.158.19:7777 \
--key hash-f9ae16d2a374e985c425c94c716c539070dc7d7907f119ba32d9684f4412a4ee \
--state-root-hash 4e8b0de303f834cb7c61bef148046e3de4446903bd15a395c9c37a6d96efe8c6
```

The named keys field, which will contain an entry called `result` with some URef associated with it.

<img src="/images/uref.png" alt="URef" title="URef">

We will use this URef value to check the balances and allowances of users within the fungible token contract.

Refer to the [token transfer](transfer.md)  guide to follow the next steps.



# Casper Fungible Token Quick Start Guide

This quick start guide introduces you to the Casper client commands and Wasm files necessary to deploy a CEP-18 Casper Fungible Token contract to a [Casper network](https://cspr.live).

The [Ethereum Request for Comment (ERC-20)](https://eips.ethereum.org/EIPS/eip-20#specification) standard defines a set of rules that dictate the total supply of tokens, how the tokens are transferred, how transactions are approved, and how token data is accessed. These fungible tokens are blockchain-based assets that have value and can be transferred or recorded.

To execute transactions on a Casper network (involving fungible tokens), you will need some CSPR tokens to pay for the transactions.

For greater detail into the creation and mechanics of the Casper fungible token contract, see the full [Casper Fungible Token Tutorial](https://github.com/casper-ecosystem/cep18/blob/master/TUTORIAL.md).

## Prerequisites

Before using this guide, ensure you meet the following requirements:

- Set up your machine as per the [prerequisites](https://docs.casper.network/developers/prerequisites/)
- Use the [Casper command-line client]
- Get a valid [`node-address`](https://cspr.live/tools/peers)
- Know how to deploy a [smart contract](https://docs.casper.network/developers/dapps/sending-deploys/) to a Casper network
- Hold enough CSPR tokens to pay for transactions

# Setup

Clone the [fungible token (CEP-18) contract repository](https://github.com/casper-ecosystem/cep18) and run the `make build-contract` command. This will create the `cep18.wasm` and the `cep18_test_contract.wasm`. The token Wasm is the main contract. We will use the `cep18_test_contract` Wasm to query the balances and allowances of the fungible token balances throughout this workflow.

## Install the Main Fungible Token Contract

The following command will create a deploy containing the CEP-18 contract instance using your supplied arguments as follows:

- **Name** - The name of your CEP-18 token
- **Symbol** - The symbol used to refer to your CEP-18 token
- **Total_supply** - The total supply of the CEP-18 token to be minted
- **Decimals** - The number of spaces after the decimal. (As an example, a total supply of 1000000 with a `decimals` setting of 3 would be 1,000.000 tokens)

```bash
casper-client put-transaction -n http://<NODE IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-path ~/casper/demo/cep18.wasm \
--session-arg "name:string='CEP18'" \
--session-arg "symbol:string='gris'" \
--session-arg "total_supply:u256='100'" \
--session-arg "decimals:u8='1'" \
--payment-amount 150000000000
```

## Install the `cep18_test_contract` Contract Package

The following command will install the CEP-18 helper contract that allows you to check balances and access approval features.

```bash
casper-client put-transaction -n http://<NODE IP>:<PORT> \
--chain-name <CHAIN NAME> \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-path ~/casper/demo/cep18_test_contract.wasm \
--payment-amount 55000000000
```

At this point, the account that installed both the main contract and the helper contract will look like this.

```bash
{
	"src": {
	"Account": {
	"_accountHash": "account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd",
	"namedKeys": [
		{
		"name": "cep18_test_contract",
		"key": "hash-999326ca8408dfd37da023eb6fd82f174151be64f83f9fb837632a0d69fd4c7e"
		},
		{
		"name": "cep18_token_contract",
		"key": "hash-b568f50a64acc8bbe43462ffe243849a88111060b228dacb8f08d42e26985180"
		},
	],
	"mainPurse": "uref-6c062525debdee18d5cad083ca530fcb65ef8741574fba4c97673f4ed00093f7-007",
	"associatedKeys": [
		{
		"accountHash": "account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd",
		"weight": 1
		}
	],
	"actionThresholds": {
		"deployment": 1,
		"keyManagement": 1
		}
		}
	}
}
```

**_Note:_**

> 1. `cep18_token_contract` is the main contract, and is a stored contract, record its hash
> 2. `cep18_test_call` is a contract package which contains the utility contract required to read the balances and allowances of users within the fungible token state.

### Next Steps

In the following sections, the sample guide explains the querying of the contract package, token transfers, and approvals.

- [Exploring the CEP18 Contracts](./2-query.md)
- [CEP-18 Token Transfers and Allowances](./3-transfer.md)
- [Testing Framework for CEP-18](./4-tests.md)

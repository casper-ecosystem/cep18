
# Using the Casper Fungible Token Contract

This guide introduces you to using a fungible token contract on the [Casper Network](https://cspr.live/).

The [Ethereum Request for Comment (ERC-20)](https://eips.ethereum.org/EIPS/eip-20#specification) standard defines a set of rules that dictate the total supply of tokens, how the tokens are transferred, how transactions are approved, and how token data is accessed. These fungible tokens are blockchain-based assets that have value and can be transferred or recorded.

We will employ the following test accounts to demonstrate the use of a fungible token contract and transfer tokens between user accounts:

-   [User A](https://integration.cspr.live/account/01f2dfc09a94ef7bce440f93a1bb6f17fdac0c913549927d452e7e91a376e9d20d)
-   [User B](https://integration.cspr.live/account/015d4e20b5f7f687be80aed6e20960898b02c7549cc49ddf583224ecd894eca375)
-   [User C](https://integration.cspr.live/account/0101fe69ae2012358e5ce8e8b39661d45d225251c4f19ebb7fc74b057637e65aa4)
-   [User D](https://integration.cspr.live/account/0171bd7bac58780ce950007de575a472bcb30457e7b68427a6ed466568d71db1d6)

To execute transactions on the Casper Network (involving fungible tokens), you will need some CSPR tokens to pay for the transactions.

To understand the implementation of a Casper fungible token contract, see the [Casper Fungible Token Tutorial](https://github.com/casper-ecosystem/erc20/blob/master/TUTORIAL.md).

## Prerequisites

Before you dive into the details of this guide, ensure you meet these requirements:

-   Set up your machine as per the [prerequisites](https://docs.casperlabs.io/workflow/setup/)
-   Use the Casper command-line client
-   Get a valid `node-address`
-   Know how to deploy a [smart contract](https://docs.casperlabs.io/dapp-dev-guide/sending-deploys/) to a Casper Network
-   Get some CSPR tokens to pay for transactions

# Setup

Clone the fungible token (ERC-20) contract repository and run the `make build-contract` command. This will create the `erc20_token.wasm` and the `erc20_test_call.wasm`. The token Wasm is the main contract. We will use the `test_call` contract wasm to query the balances and allowances of the fungible token balances throughout this workflow.

## Install the Main Fungible Token Contract

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--chain-name integration-test \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-path ~/casper/demo/erc20_token.wasm \
--session-arg "name:string='ERC20'" \
--session-arg "symbol:string='gris'" \
--session-arg "total_supply:u256='100'" \
--session-arg "decimals:u8='1'" \
--payment-amount 90000000000
```

## Install the erc20_test_call Contract Package

```bash
casper-client put-deploy -n http://3.143.158.19:7777 \
--chain-name integration-test \
--secret-key ~/casper/demo/user_a/secret_key.pem \
--session-path ~/casper/demo/erc20_test_call.wasm \
--payment-amount 90000000000
```

At this point, the account that installed both the main contract and the helper contract will look like this.

```bash
{
	"src": {
	"Account": {
	"_accountHash": "account-hash-303c0f8208220fe9a4de40e1ada1d35fdd6c678877908f01fddb2a56502d67fd",
	"namedKeys": [
		{
		"name": "erc20_test_call",
		"key": "hash-999326ca8408dfd37da023eb6fd82f174151be64f83f9fb837632a0d69fd4c7e"
		},
		{
		"name": "erc20_token_contract",
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

> 1. `erc20_token_contract` is the main contract, and is a stored contract, record its hash
> 2. `erc20_test_call` is a contract package which contains the utility contract required to read the balances and allowances of users within the fungible token state.

### Next Steps
In the following sections, sample guide explains the querying of the contract package, token transfers and approvals, and final balance checks with code samples. 
- [Query the contract package](query.md)
- [Token transfer](transfer.md)
- [Token approval](approve.md)
- [Check final balance](final-balance.md)



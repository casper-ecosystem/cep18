# Deploying an CEP-18 Compliant Token on Casper

On the Casper Network, it is possible to create smart contracts that emulate CEP-18 tokens on Ethereum. These tokens have all of the capabilities of traditional CEP-18 tokens, allowing one to approve, transfer, inquire the balance of, etc. By following this guide, you'll be able to create your own CEP-18 token and deploy it to the Casper Network.

## Prerequisites

- [Set up an account](https://docs.casperlabs.io/workflow/setup#setting-up-an-account).
- [Fund your account](https://docs.casperlabs.io/workflow/setup#fund-your-account).

- You will need to have [Node.js](https://nodejs.org/en/) installed. Follow the [instructions](https://nodejs.org/en/download) to install it on your system.

- Lastly, you'll need at least a basic understanding of the [bash command line](https://www.gnu.org/software/bash/manual/bash.html). *`zsh` and other related shells should work as well.*

## Cloning the JavaScript Interface

We'll be starting in the home (`~`) directory for this tutorial. Navigate here first:

`cd ~`

Now clone the prewritten [JavaScript deployer project](https://github.com/casper-ecosystem/casper-cep18-js-interface), so you don't have to reinvent the wheel. This project includes a precompiled Casper fungible token contract with only basic functionality. If you'd like to write your own fungible token contract with custom logic, please follow the instructions [here](https://docs.casperlabs.io/writing-contracts). Execute this command in your home directory:

`git clone https://github.com/casper-ecosystem/casper-cep18-js-interface.git`

Change your working directory to the project directory:

`cd casper-cep18-js-interface`

Install the required dependencies:

`npm install`

## Edit the Deployment Script

Start by opening the file `index.js` in your preferred text editor or IDE.

We'll start by redefining our constants. These start on line `5` and should look like the following:

```javascript
const NAME = "Test Token";
const SYMBOL = "TST";
const PRECISION = 8;
const TOTAL_SUPPLY = 1_000_000_000;
const GAS_LIMIT = 60_000_000_000; //motes
const WASM_PATH = "./cep18_token.wasm";
const NODE_ADDRESS = "http://162.55.132.188:7777/rpc";
const NETWORK_NAME = "casper-test";
```

Let's take a look at what these constants refer to.

* `NAME`: The name of the fungible token contract.
* `SYMBOL`: The symbol of the Casper fungible token contract.
* `PRECISION`: The number of decimal places the token can be fractionalized to.
* `TOTAL_SUPPLY`: The total supply of your fungible token token.
* `GAS_LIMIT`: The gas limit in motes that will be used to pay for the deployment.
* `WASM_PATH`: The path to the compiled contract.
* `NODE_ADDRESS`: The validator node used to submit the deploy. The address listed directs to a valid online node, but this may change in the future. If this node does not respond, you can select another online peer from the list [here](https://testnet.cspr.live/tools/peers). Note that you'll need to replace the port with `7777` for most nodes and add `/rpc` to the end of the address.
* `NETWORK_NAME`: The name of the network to which you'll be deploying. By default, we have the Casper Testnet specified as `"casper-test"`. To deploy on the Mainnet, you may change this to `"casper"`.

As long as you generated the keys with the aforementioned command within your project's root folder, the paths to your keys should be the same as already written in the code. Otherwise, you'll need to put in the alternate path to your keys in the `KEYS` constant.

```javascript
const KEYS = Keys.Ed25519.loadKeyPairFromPrivateFile(
  "./keys/secret_key.pem"
);
```

## Install the Contract

To install the contract, execute the following command:

`npm run cep18iface deploy`

You'll know your deployment succeeded when the name of the contract is printed out in the console.

`... Contract name: Test Token`

Your Casper fungible token contract is now successfully installed. Next, we'll go about transferring tokens.

**Note:**

An error message will be provided if a deploy fails. Deployments may fail for a number of reasons, including out-of-gas errors, and internet connectivity issues.

*For help, run `npm run cep18iface help`*

## Transfer Tokens

You can send CEP-18 compliant tokens on the Casper Network as per the ERC specification. We'll use the same  `cep18iface` script to execute the transfer deployments.

You will need an amount and a destination to execute a transfer. The amount corresponds to the number of tokens you want to transfer, and the destination is the hexadecimal public key of the receiving account. Your command should look like this:

`npm run cep18iface transfer 200 0166795bb8895dcec5631690fa3d5dd3daacde7efeefb1e79176e9d879fd669b47`

To send tokens from the address you just funded, just change the `KEYS` constant to use the path of that account's public and secret keys (in this case account `0166795bb8895dcec5631690fa3d5dd3daacde7efeefb1e79176e9d879fd669b47`).

## Conclusion

Thank you for reading Casper's fungible token deployment guide. Next, you can learn how to write your own [Casper fungible token smart contracts](TUTORIAL.md), and use [Casper's JavaScript SDK](https://docs.casperlabs.io/dapp-dev-guide/sdk/script-sdk).


# Deploying a CEP-18 Compliant Token with the JS SDK

On Casper networks, CEP18 compliant tokens emulate the features available from ERC20 tokens on the Ethereum network. These tokens feature all the capabilities of traditional ERC20 tokens and by following this guide, you will be able to create your own CEP-18 token and deploy it to a Casper network.

## Prerequisites

- [Set up an account](https://docs.casper.network/concepts/accounts-and-keys/#creating-accounts-and-keys).
- [Fund your account](https://docs.casper.network/concepts/accounts-and-keys/#funding-your-account).

- You will need to have [Node.js](https://nodejs.org/en/) installed. Follow the [instructions](https://nodejs.org/en/download) to install it on your system.

- Lastly, you'll need at least a basic understanding of the [bash command line](https://www.gnu.org/software/bash/manual/bash.html). *`zsh` and other related shells should work as well.*

## Cloning the JavaScript Interface

We will start in the home (`~`) directory for this tutorial. Navigate here first:

`cd ~`

Now clone the prewritten [JavaScript deployer project](https://github.com/casper-ecosystem/casper-erc20-js-interface), so you don't have to reinvent the wheel. This project includes a precompiled Casper fungible token contract with only basic functionality. If you'd like to write your own fungible token contract with custom logic, please follow the instructions [here](https://docs.casper.network/developers/writing-onchain-code/simple-contract/). Execute this command in your home directory:

`git clone https://github.com/casper-ecosystem/casper-erc20-js-interface.git`

Change your working directory to the project directory:

`cd casper-erc20-js-interface`

Install the required dependencies:

`npm install`

## Edit the Deployment Script

Start by opening the file `index.js` in your preferred text editor or IDE.

Then redefine your constants, which start on line `5` and should look like the following:

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

These constants refer to the following required information:

* `NAME`: The name of your Casper fungible token.
* `SYMBOL`: The symbol used to refer to your token.
* `PRECISION`: The number of decimal places the token can be fractionalized to.
* `TOTAL_SUPPLY`: The total supply of your fungible token token.
* `GAS_LIMIT`: The gas limit in motes that will be used to pay for the deployment.
* `WASM_PATH`: The path to the compiled contract.
* `NODE_ADDRESS`: The validator node used to submit the deploy. The address listed directs to a valid online node, but this may change in the future. If this node does not respond, you can select another online peer from the list [here](https://testnet.cspr.live/tools/peers). Note that you'll need to replace the port with `7777` for most nodes and add `/rpc` to the end of the address.
* `NETWORK_NAME`: The name of the network to which you'll be deploying. By default, we have the Casper Testnet specified as `"casper-test"`. To deploy on the Mainnet, you may change this to `"casper"`.

As long as you generated the keys with the aforementioned command within your project's root folder, the paths to your keys should be the same as already written in the code. Otherwise, you will need to put in the alternate path to your keys in the `KEYS` constant.

```javascript
const KEYS = Keys.Ed25519.loadKeyPairFromPrivateFile(
  "./keys/secret_key.pem"
);
```

## Install the Contract

To install the contract, execute the following command:

`npm run erc20iface deploy`

The name of your contract will print in the console when the deploy succeeds.

`... Contract name: Test Token`

Your Casper fungible token contract instance is now successfully installed. Next, we will cover transferring tokens.

**Note:**

An error message will be provided if a deploy fails. Deploys may fail for a number of reasons, including out-of-gas errors or internet connectivity issues.

*For help, run `npm run erc20iface help`*

## Transfer Tokens

You can send ERC20 compliant tokens on Casper networks as per the ERC specification. We use the same  `erc20iface` script to execute the transfer deploy.

You will need an amount and a destination to execute a transfer. The amount corresponds to the number of tokens you want to transfer, and the destination is the hexadecimal public key of the receiving account. Your command should look like this:

`npm run erc20iface transfer 200 0166795bb8895dcec5631690fa3d5dd3daacde7efeefb1e79176e9d879fd669b47`

To send tokens from the address you just funded, change the `KEYS` constant to use the path of that account's public and secret keys (in this case account `0166795bb8895dcec5631690fa3d5dd3daacde7efeefb1e79176e9d879fd669b47`).


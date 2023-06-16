# `casper-cep18-js-client`

This JavaScript client gives you an easy way to install and interact with the Casper CEP-18 contract.

## Installation

Run this command to install the client:

```bash
npm install casper-js-sdk @make-software/ces-js-parser casper-cep18-js-client
```

The `casper-cep18-js-client` requires `casper-js-sdk` and `@make-software/ces-js-parser` as a peer dependency.

## Usage Examples

Create an instance of the CEP-18 client:

```ts
import { ContractWASM, CEP18Client } from 'casper-cep18-js-client';

const NODE_URL = 'http://localhost:11101/rpc';
const NETWORK_NAME = 'casper-net-1';

const cep18 = new CEP18Client(NODE_URL, NETWORK_NAME);
```

Create a deploy to install the contract:

```ts
const deploy = cep18.install(
  ContractWASM, // Contract wasm
  {
    name: 'TEST',
    symbol: 'TST',
    decimals: 9,
    totalSupply: 50_000_000_000
  },
  60_000_000_000, // Payment Amount
  ownerPublicKey,
  NETWORK_NAME,
  [owner]
);
```

Set the contract hash (a unique identifier for the network):

```ts
cep18.setContractHash(
  'hash-c2402c3d88b13f14390ff46fde9c06b8590c9e45a9802f7fb8a2674ff9c1e5b1'
);
```

You can retrieve token information by calling these methods:

```ts
const name = await cep18.name();

const symbol = await cep18.symbol();

const totalSupply = await cep18.totalSupply();

const decimals = await cep18.decimals();
```

**Transfers**

Create a deploy to transfer some tokens from the direct caller to a recipient:

```ts
const deploy = cep18.transfer(
  { recipient: recipientPublicKey, amount: 50_000_000_000 },
  5_000_000_000, // Payment amount
  ownerPublicKey,
  NETWORK_NAME,
  [ownerAsymmetricKey] // Optional
);
```

Create a deploy to transfer from an account owner to a recipient, given that the direct caller has been previously approved to spend the specified amount on behalf of the owner:

```ts
const deploy = cep18.transferFrom(
  {
    owner: ownerPublicKey,
    recipient: recipientPublicKey,
    amount: transferAmount
  },
  5_000_000_000,
  approvedPublicKey,
  NETWORK_NAME,
  [approvedAsymmetricKey]
);
```

**Balances**

Request the balance of an account with _balanceOf_:

```ts
const balance = await cep18.balanceOf(publicKey);
```

**Approvals**

Create a deploy to allow a spender to transfer up to a number of the direct caller’s tokens:

```ts
const deploy = cep18.approve(
  {
    spender: spenderPublicKey,
    amount: approveAmount
  },
  5_000_000_000,
  ownerPublicKey,
  NETWORK_NAME,
  [ownerAsymmetricKey]
);
```

**Allowances**

Return the number of owner’s tokens allowed to be spent by spender:

```ts
const allowance = await cep18.allowances(
  ownersPublicKey,
  spenderPublicKey
);
```

To increase or decrease the spender's allowance, use the following methods:

```ts
const deploy = cep18.increaseAllowance(
  {
    spender,
    amount
  },
  5_000_000_000,
  owner.publicKey,
  NETWORK_NAME,
  [owner]
);
```

```ts
const deploy = cep18.decreaseAllowance(
  {
    spender,
    amount
  },
  5_000_000_000,
  owner.publicKey,
  NETWORK_NAME,
  [owner]
);
```

> The `mint`, `burn`, and `changeSecurity` deploy maybe failed if mint and burn is disabled in the contract. **You can only enable mint and burn when install contract.** You can check mint and burn is enabled by running
>
> ```ts
> const isMintAndBurnEnabled = await cep18.isMintAndBurnEnabled();
> ```

**Minting Tokens**

Mint tokens and assign them to a recipient:

```ts
const deploy = cep18.mint(
  {
    owner: recipient,
    amount
  },
  5_000_000_000,
  owner.publicKey,
  NETWORK_NAME,
  [owner]
);
```

**Burning Tokens**

Burn tokens and reduce them from the owner's account:

```ts
const deploy = cep18.burn(
  {
    owner: recipient,
    amount
  },
  5_000_000_000,
  owner.publicKey,
  NETWORK_NAME,
  [owner]
);
```

**Changing User Security**

```ts
const minterList = [ali.publicKey];
const burnerList = [ali.publicKey, bob.publicKey];
const deploy = cep18.changeSecurity(
  {
    adminList: [owner.publicKey],
    minterList,
    burnerList
  },
  5_000_000_000,
  owner.publicKey,
  NETWORK_NAME,
  [owner]
);
```

## Event Handling

CEP-18 tokens support the [Casper Event Standard (CES)](https://github.com/make-software/casper-event-standard), and tokens can be installed with or without event logging as described [here](../cep18/README.md#eventsmode). If you install a token with the EventsMode set to CES, you can listen to token events using the `EventStream` from the `casper-js-sdk`. To consume token events, you should also install the `@make-software/ces-js-parser` by running this command:

```bash
npm install @make-software/ces-js-parser
```

Set up the `EventStream`:

```ts
import { EventStream } from 'casper-js-sdk';
import { CEP18Client } from 'casper-cep18-js-client';

const cep18 = new CEP18Client(
  'http://localhost:11101/rpc', // Node address
  'casper-net-1' // Network name
);
cep18.setContractHash(
  `hash-0885c63f5f25ec5b6f3b57338fae5849aea5f1a2c96fc61411f2bfc5e432de5a`
);
await cep18.setupEventStream(
  new EventStream('http://localhost:18101/events/main')
);
```

Here is how you can consume events using event listeners.

- Add an event listener:

```ts
const listener = event => {
  console.log(event.name); // 'Burn'
  console.log(event.data); // Burn event info
};

cep18.on('Burn', listener);
```

- Remove an event listener:

```ts
cep18.off('Burn', listener);
```

## More examples

Additional examples are in available in the [examples](https://github.com/casper-ecosystem/cep18/tree/dev/client-js/examples), and [tests](https://github.com/casper-ecosystem/cep18/tree/dev/client-js/tests) directory.

## Development

Before installing the node modules, make sure the contract Wasm is generated by running the following:

```bash
make build-contracts
```

After generating the Wasm file, you can install the node modules, and the Wasm will be automatically bundled.

```bash
npm install && npm run generate:wasm
```

## Testing

Unit and integration tests are available in the `client-js` directory.

First, you must clone the repository:

```bash
git clone https://github.com/casper-ecosystem/cep18.git
```

Go to the `client-js` directory:

```bash
cd client-js
```

Intall the node modules using the following:

```bash
npm install && npm run generate:wasm
```

Run unit tests:

```bash
npm run test:unit
```

You can test the script by running a [local network](https://github.com/casper-network/casper-node/blob/dev/utils/nctl/README.md). After setting up the local network, run the end-to-end integration tests with this command:

```bash
npm run test:e2e
```

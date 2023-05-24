# `casper-cep18-js-client`

This JavaScript client gives you an easy way to install and interact with the Casper CEP-18 contract.

## Installation

Run this command to install the client:

> **NOTE**
>
> The `casper-cep18-js-client` requires `casper-js-sdk` as peer dependency

```bash
npm install casper-js-sdk casper-cep18-js-client
```

## Usage example

- Create an instance of the CEP-18 client:

  ```ts
  import { ContractWASM, CEP18Client } from 'casper-cep18-js-client';

  const NODE_URL = 'http://localhost:11101/rpc';
  const NETWORK_NAME = 'casper-net-1';

  const cep18 = new CEP18Client(NODE_URL, NETWORK_NAME);
  ```

- Create a deploy to install the contract:

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

- Set the contract hash (a unique identifier for the network):

  ```ts
  cep18.setContractHash(
    'hash-c2402c3d88b13f14390ff46fde9c06b8590c9e45a9802f7fb8a2674ff9c1e5b1'
  );
  ```

- You can retrieve token infomation by calling these methods:

  ```ts
  const name = await cep18.name();

  const symbol = await cep18.symbol();

  const totalSupply = await cep18.totalSupply();

  const decimals = await cep18.decimals();
  ```

- **Transfers**

  - Create a deploy to transfer some tokens from the direct caller to a recipient:

    ```ts
    const deploy = cep18.transfer(
      { recipient: recipientPublicKey, amount: 50_000_000_000 },
      5_000_000_000, // Payment amount
      ownerPublicKey,
      NETWORK_NAME,
      [ownerAsymmetricKey] // Optional
    );
    ```

  - Create a deploy to transfer from an account owner to a recipient given that the direct caller has been previously approved to spend the specified amount on behalf of the owner:

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

- **Balances**

  Request the balance of an account with _balanceOf_:

  ```ts
  const balance = await cep18.balanceOf(publicKey);
  ```

- **Approvals**

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

- **Allowance**

  Return the number of owner’s tokens allowed to be spent by spender:

  ```ts
  const allowance = await cep18.allowances(
    ownersPublicKey,
    spenderPublicKey
  );
  ```

## Event Handling

CEP18 token supports [CES](https://github.com/make-software/casper-event-standard) and the token can be intalled with CES or NoCES method. If the token is installed with CES method, you can listen token events by using `EventStream` from `casper-js-sdk`. To consume token events, you should install `@make-software/ces-js-parser` by running

```bash
npm install @make-software/ces-js-parser
```

- Setup EventStream

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

- Consume events

  - Add event listener

  ```ts
  const listener = event => {
    console.log(event.name); // 'Burn'
    console.log(event.data); // Burn event info
  };

  cep18.on('Burn', listener);
  ```

  - Remove event listener

  ```ts
  cep18.off('Burn', listener);
  ```

## More examples

You can find all the available examples in the [E2E test script](https://github.com/casper-ecosystem/cep18/blob/master/client-js/tests/e2e/).

## Development

Before install node modules, make sure the wasm file is generated.

- You can generate wasm file by running

  ```bash
  make build-contracts
  ```

- After generate wasm file, you can install node modules and the wasm will be automatically bundled.

  ```bash
  npm install
  ```

## Test

- Clone this repo

```bash
git clone https://github.com/casper-ecosystem/cep18.git
```

- Go to `client-js` directory

```bash
cd client-js
```

- Intall modules using

```bash
npm install
```

- Run unit test by

```bash
npm run test:unit
```

- You can test the script by running the [local network](https://github.com/casper-network/casper-node/blob/dev/utils/nctl/README.md). After running the local network run the test by

```bash
npm run test:e2e
```

# `casper-cep18-js-client`

This JavaScript client gives you an easy way to install and interact with the Casper CEP-18 contract.

## Installation

Run this command to install the client:

```bash
npm i casper-cep18-js-client
```

## Usage example

- Create an instance of the CEP-18 client:

  ```ts
  const cep18 = new CEP18Client(
    'http://localhost:11101/rpc', // Node address
    'casper-net-1' // Network name
  );
  ```

- Create a deploy to install the contract:

  ```ts
  import { ContractWASM, CEP18Client } from 'casper-cep18-js-client';

  const cep18 = new CEP18Client(NODE_URL, NETWORK_NAME);
  const deploy = cep18.install(
    ContractWASM, // Contract wasm
    {
      name: tokenName,
      symbol: tokenSymbol,
      decimals: tokenDecimals,
      totalSupply: totalSupply
    },
    60_000_000_000, // Payment Amount
    ownerPublicKey,
    CHAIN_NAME,
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
      CHAIN_NAME,
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
      CHAIN_NAME,
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
    CHAIN_NAME,
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

## More examples

You can find all the available examples in the [E2E test script](https://github.com/casper-ecosystem/cep18/client-js/tests/e3e).

## Development

Before working on the JS Client, make sure the bundled contract content in the [`wasm.ts`]('./src.wasm.ts') is up to date with the current wasm file.

- You can generate wasm file by running

  ```bash
  make build-contracts
  ```

- After generate wasm file, you can bundle it by running

  ```bash
  npm run generate:wasm
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

- You can test the script by running the [local network](https://github.com/casper-network/casper-node/blob/dev/utils/nctl/README.md). After running the local network run the test by

```bash
npm test
```

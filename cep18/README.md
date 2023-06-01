# CEP-18: The Casper Fungible Token Standard

## Table of Contents

1. [Modalities](#modalities)

2. [Entry Points](#entry-points)

3. [Testing](#testing)

4. [Error Codes](#error-codes)

5. [Usage](#usage)

## Modalities

The fungible token implementation supports a couple of `modalities` that dictate the behavior of a specific contract instance. Modalities represent the common expectations around contract usage and behavior. The following section discusses the modalities currently available.

<!-- TODO verify that these modalities canNOT be changed after installation. -->

### EventsMode

The `EventsMode` modality determines how the installed instance of CEP-18 will handle the recording of events that occur from interacting with the contract. The mode is set by passing a `u8` value to the `events_mode` runtime argument: `--session-arg "events_mode:u8='1'"`. The default behavior is `NoEvents`.

**IMPORTANT: This mode cannot be changed once the contract has been installed.**

The modality provides two options:

1. `NoEvents`: This modality will signal the contract not to record events. This is the default mode.
2. `CES`: This modality will signal the contract to record events using the [Casper Event Standard (CES)](#casper-event-standard).

| EventsMode | u8  |
| ---------- | --- |
| NoEvents   | 0   |
| CES        | 1   |


#### The Casper Event Standard

`CES` is an option within the `EventsMode` modality that determines how changes to tokens issued by the contract instance will be recorded. Changes are recorded in the `__events` dictionary and can be observed via a node's Server Side Events stream. They may also be viewed by querying the dictionary at any time using the JSON-RPC interface.

<!-- TODO who creates this dictionary? Is it created and managed by the casper_event_standard crate? -->

The emitted events are encoded according to the [Casper Event Standard](https://github.com/make-software/casper-event-standard), and the schema is visible to an observer reading the `__events_schema` contract named key.

For this CEP-18 reference implementation, the events schema is as follows:

<!-- TODO check this table with the dev team. I've extracted from src/events.rs -->

| Event name        | Included values and their type                                 |
| ----------------- | -------------------------------------------------------------- |
| Mint              | recipient (Key), amount (U256)                                 |
| Burn              | owner (Key), amount (U256)                                     |
| SetAllowance      | owner (Key), spender (Key), allowance (U256)                   |
| IncreaseAllowance | owner (Key), spender (Key), allowance (U256), inc_by (U256)    |
| DecreaseAllowance | owner (Key), spender (Key), allowance (U256), decr_by (U256)   |
| Transfer          | sender (Key), recipient (Key), amount (U256)                   |
| TransferFrom      | spender (Key), owner (Key), recipient (Key), amount (U256)     |
| ChangeSecurity    | pub admin (Key), sec_change_map (BTreeMap<Key, SecurityBadge>) |


### MintBurn

The `MintBurn` modality dictates whether tokens managed by a given instance of a CEP-18 contract can be minted or burnt after contract installation. 

**IMPORTANT: This mode cannot be changed once the contract has been installed.**

This modality provides two options:

1. `Disabled`: Tokens cannot be minted nor burnt after contract installation. This is the default mode.
2. `MintAndBurn`: Tokens can be minted and burnt.

| MintBurn    | u8  |
| ----------- | --- |
| Disabled    | 0   |
| MintAndBurn | 1   |

This modality is specified by providing an optional runtime argument during installation. The mode is set by passing a `u8` value to the `enable_mint_burn` runtime argument: `--session-arg "enable_mint_burn:u8='1'"`. The default behavior is `Disabled`.

### Example Installation

Here is a sample deploy installing a fungible token with event logging and minting and burning enabled:

```bash
casper-client put-deploy \
--node-address http://65.21.235.219:7777  \
--chain-name casper-test \
--secret-key ~/KEYS/secret_key.pem \
--payment-amount 120000000000 \
--session-path ./target/wasm32-unknown-unknown/release/cep18.wasm \
--session-arg "name:string='Test Token'" \
--session-arg "symbol:string='DEMO'" \
--session-arg "total_supply:u256='100'" \
--session-arg "decimals:u8='2'" \
--session-arg "events_mode:u8='1'" \
--session-arg "enable_mint_burn:u8='1'"
```

## Entry Points

The Casper CEP-18 Standard follows the [ERC20 Standard](https://eips.ethereum.org/EIPS/eip-20) by implementing the IERC20 interface. The explanations below are summarized from the ERC20 set of interfaces, contracts, and utilities found [here](https://docs.openzeppelin.com/contracts/4.x/api/token/erc20).

* `init` - Entrypoint called only once during contract installation.
* `allowance` - Returns the number of tokens that a spender can spend on behalf of the owner. The default is zero until `approve` or `transferFrom` are called.
* `increase_allowance` - Increases the allowance granted to a spender by the caller. This is an alternative to `approve`.
* `decrease_allowance` - Decreases the allowance granted to a spender by the caller. This is an alternative to `approve`.
* `approve` - Sets a spender's allowance over the caller’s tokens.
* `balance_of` - Returns the number of tokens owned by the account specified.
* `decimals` - Returns the number of decimals used to represent the token to a user. For example, if `decimals` equals `2`, a balance of `505` tokens should be displayed to a user as `5.05`.
* `name` - Returns the name of the token.
* `symbol` - Returns the symbol of the token, usually a shorter version of the name; for example, CSPR.
* `total_supply` - Returns the number of tokens in existence.
* `transfer` - Moves tokens from the caller to the specified recipient. 
* `transfer_from` - Moves tokens from the owner to a recipient if the caller has been approved to spend the owner's tokens.
* `mint` - Creates the number of tokens specified and assigns them to an account, increasing the total supply.
* `burn` - Destroys the number of tokens specified from an account, reducing the total supply.
* `change_security` - An entrypoint specific to CEP-18, used for Administration and Security operations. See more details below.

### Changing Security Access

The `change_security` entrypoint manages the security access granted to users. One user can only possess one access group badge. The groups and the change strength are: 

* None > Admin > MintAndBurn > Burner > Minter

For example, if a user is added to both Minter and Admin, they will be an Admin.
If a user is added to Admin and None, they will be removed from having access rights.

**IMPORTANT: do NOT remove the last Admin, because that will lock out all admin functionality.**

## Testing

This repository contains several ways of testing the fungible token contract and its entrypoints.

1. The test suite found in the [tests](../tests/) folder asserts the expected behavior of the contract implementation. It also ensures that no regressions and conflicting behaviors are introduced as functionality is added or extended. The tests can be run by using the provided `Makefile` and running the `make test` command.

2. A test contract that calls the entrypoints in the fungible token contract is also available in the [cep18-test-contract](../cep18-test-contract/) folder.

3. The JavaScript client in the [client-js](../client-js/README.md) folder has unit tests and end-to-end integration tests.

4. A set of benchmarking scripts is also available in the [cost-benchmarking](../cost-benchmarking/README.md) folder, for testing gas costs of basic operations.


## Error Codes

<!-- TODO check with the dev team if these explanations are correct. I've extracted from the code. -->

The table below summarizes the [error codes](./src/error.rs) you may see while working with fungible tokens.

| Code  | Error                  | Description                                             |
| ----- | ---------------------- | --------------------------------------------------------|
| 60000 | InvalidContext         | The contract was called from within an invalid context. |
| 60001 | InsufficientBalance    | The spender does not have enough funds.                 |
| 60002 | InsufficientAllowance  | The spender does not have enough allowance approved.    |
| 60003 | Overflow               | This operation would cause an integer overflow.         |
| 60004 | PackageHashMissing     | A required package hash was not specified.              |
| 60005 | PackageHashNotPackage  | The package hash specified does not represent a package.|
| 60006 | InvalidEventsMode      | An invalid event mode was specified.                    |
| 60007 | MissingEventsMode      | The event mode required was not specified.              |
| 60008 | Phantom                | An unknown error occurred.                              |
| 60009 | FailedToGetArgBytes    | Failed to read the runtime arguments provided.          |
| 60010 | InsufficientRights     | The caller does not have sufficient security access.    |
| 60011 | InvalidAdminList       | The list of Admin accounts provided is invalid.         |
| 60012 | InvalidMinterList      | The list of accounts that can mint tokens is invalid.   |
| 60013 | InvalidBurnerList      | The list of accounts that can burn tokens is invalid.   |
| 60014 | InvalidMintAndBurnList | The list of accounts that can mint and burn is invalid. |
| 60015 | InvalidNoneList        | The list of accounts with no access rights is invalid.  |
| 60016 | InvalidEnableMBFlag    | The flag to enable the mint and burn mode is invalid.   |
| 60017 | AlreadyInitialized     | This contract instance cannot be initialized again.     |
| 60018 | MintBurnDisabled       | The mint and burn mode is disabled.                     |

### Usage

For installing and interacting with the fungible token reference contract, read the examples provided in the [docs](../docs) folder.

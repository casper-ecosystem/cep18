import {
  CEP18Client,
  CEP18_EVENTS,
  CEP18EventResult,
  EVENTS_MODE,
  TransactionParams,
  ChangeSecurityArgs
} from 'src';
import { expect, describe, it, beforeEach } from 'vitest';
import { RPC_URL, SSE_URL, CHAIN_NAME } from '../../config';
import { getAccountInfo, findKeyFromAccountNamedKeys } from '../utils';
import { owner, install, mint, ali } from './helpers';

let client: CEP18Client;
const name = `TEST_CEP18_E2E_${Math.floor(Math.random() * 1000000)}`,
  waitForTransactionProcessed = false; // Do not wait for transactions execution, avoid duplicate listener of event

describe('CEP18Client - Event Streaming', () => {
  beforeEach(async () => {
    client = new CEP18Client(RPC_URL, SSE_URL, CHAIN_NAME);
    await install(client, name);
    const account = await getAccountInfo(RPC_URL, owner.publicKey),
      contractHash = findKeyFromAccountNamedKeys(
        account,
        `cep18_contract_hash_${name}`
      );
    expect(contractHash).toBeDefined();
    client.setContractHash(contractHash);
  }, 180000);

  it('should start and stop event stream and listen to events when on() is called', async () => {
    // Start the event stream
    client.startEventStream();

    const mintEvent = CEP18_EVENTS.Mint;
    let mintEventReceived = false;

    // Mint tokens and listen for the Mint event
    const mintAmount = BigInt(10_000_000_000);
    await mint(client, mintAmount, waitForTransactionProcessed);

    // Listen for Mint event in a promise awaited
    await new Promise<CEP18EventResult>(resolve => {
      client.on(mintEvent, async eventResult => {
        mintEventReceived = true;
        resolve(eventResult);
      });
    });

    expect(mintEventReceived).toBe(true);

    // Stop the event stream
    client.stopEventStream();

    // Try to mint tokens again, but event listener shouldn't trigger
    await mint(client, mintAmount, waitForTransactionProcessed);

    let eventFired = false;
    client.on(mintEvent, () => {
      eventFired = true;
    });

    // The listener shouldn't fire because stream is stopped
    setTimeout(() => {
      expect(eventFired).toBe(false);
    }, 1000);
  }, 180000);

  it('should remove a specific event listener using off()', async () => {
    client.startEventStream();
    const mintEvent = CEP18_EVENTS.Mint;
    let eventTriggered = false;

    const listener = () => {
      eventTriggered = true;
    };

    client.on(mintEvent, listener);
    client.off(mintEvent, listener); // Remove the listener

    await mint(client, BigInt(10_000_000_000), waitForTransactionProcessed);

    setTimeout(() => {
      expect(eventTriggered).toBe(false);
    }, 1000);

    client.stopEventStream();
  }, 180000);

  it('should remove all listeners for a specific event using removeListenersForEvent()', async () => {
    client.startEventStream();
    const mintEvent = CEP18_EVENTS.Mint;
    let firstListenerTriggered = false;
    let secondListenerTriggered = false;

    client.on(mintEvent, () => {
      firstListenerTriggered = true;
    });

    client.on(mintEvent, () => {
      secondListenerTriggered = true;
    });

    client.removeListenersForEvent(mintEvent); // Remove all listeners for Mint

    await mint(client, BigInt(10_000_000_000), waitForTransactionProcessed);

    setTimeout(() => {
      expect(firstListenerTriggered).toBe(false);
      expect(secondListenerTriggered).toBe(false);
    }, 1000);

    client.stopEventStream();
  }, 180000);

  it('should remove all event listeners using removeAllListeners()', async () => {
    client.startEventStream();
    let mintTriggered = false;
    let burnTriggered = false;

    client.on(CEP18_EVENTS.Mint, () => {
      mintTriggered = true;
    });

    client.on(CEP18_EVENTS.Burn, () => {
      burnTriggered = true;
    });

    client.removeAllListeners(); // Remove all event listeners

    await mint(client, BigInt(10_000_000_000), waitForTransactionProcessed);

    setTimeout(() => {
      expect(mintTriggered).toBe(false);
      expect(burnTriggered).toBe(false);
    }, 1000);

    client.stopEventStream();
  }, 180000);
});

describe('CEP18Client - Events emit', () => {
  let client: CEP18Client;

  beforeEach(async () => {
    client = new CEP18Client(RPC_URL, SSE_URL, CHAIN_NAME);
    await install(client, name);

    const account = await getAccountInfo(RPC_URL, owner.publicKey);
    const contractHash = findKeyFromAccountNamedKeys(
      account,
      `cep18_contract_hash_${name}`
    );

    expect(contractHash).toBeDefined();
    client.setContractHash(contractHash);
  }, 180000);

  it('should emit EventsModeChanged when events mode is changed', async () => {
    let currentMode = await client.eventsMode();
    expect(currentMode).toBe('CES');

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(3_000_000_000),
      signingKeys: [owner]
    };

    let newMode = EVENTS_MODE.NoEvents;

    await client.changeEventsMode({
      params,
      args: { eventsMode: newMode },
      waitForTransactionProcessed: true
    });

    currentMode = await client.eventsMode();
    expect(currentMode).toBe('NoEvents');

    client.startEventStream();

    newMode = EVENTS_MODE.CES;

    await client.changeEventsMode({
      params,
      args: { eventsMode: newMode },
      waitForTransactionProcessed: false
    });

    const changeEventsModeEvent = CEP18_EVENTS.ChangeEventsMode;
    let eventReceived = false;
    await new Promise<CEP18EventResult>(resolve => {
      client.on(changeEventsModeEvent, async eventResult => {
        eventReceived = true;
        resolve(eventResult);
      });
    });

    expect(eventReceived).toBe(true);

    client.stopEventStream();

    currentMode = await client.eventsMode();
    expect(currentMode).toBe('CES');
  }, 180000);

  it('should emit ChangeSecurity when security is changed', async () => {
    let currentMode = await client.eventsMode();
    expect(currentMode).toBe('CES');

    const newAdminList = [ali.publicKey];

    const params: TransactionParams = {
      sender: owner.publicKey,
      paymentAmount: String(5_000_000_000),
      signingKeys: [owner]
    };

    const changeSecurityArgs: ChangeSecurityArgs = {
      adminList: newAdminList
    };

    client.startEventStream();

    await client.changeSecurity({
      params,
      args: changeSecurityArgs,
      waitForTransactionProcessed: false
    });

    const changeSecurityEvent = CEP18_EVENTS.ChangeSecurity;
    let eventReceived = false;
    await new Promise<CEP18EventResult>(resolve => {
      client.on(changeSecurityEvent, async eventResult => {
        eventReceived = true;
        resolve(eventResult);
      });
    });

    expect(eventReceived).toBe(true);

    client.stopEventStream();
  }, 180000);
});

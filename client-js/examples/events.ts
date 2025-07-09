import {
  type MintArgs,
  type TransactionParams,
  CEP18Client,
  type BurnArgs,
  CEP18_EVENTS,
  CEP18EventResult,
  InfoGetTransactionResult,
  EVENTS_MODE
} from 'dist';
import {
  PRIVATE_KEY_FAUCET,
  SSE_URL,
  PRIVATE_KEY_USER_1,
  CHAIN_NAME,
  RPC_URL
} from '../config';
import {
  findKeyFromAccountNamedKeys,
  getAccountInfo,
  getSigningKey
} from '../tests/utils';

// Here you can check examples how to mint and burn tokens and listen to event stream

if (!PRIVATE_KEY_FAUCET) {
  throw new Error('FAUCET_SECRET_KEY environment variable is not set.');
}
if (!PRIVATE_KEY_USER_1) {
  throw new Error('PRIVATE_KEY_USER_1 environment variable is not set.');
}

const name = 'TEST_CEP18',
  owner = getSigningKey(PRIVATE_KEY_FAUCET),
  ali = getSigningKey(PRIVATE_KEY_USER_1);

const usage = async () => {
  const account = await getAccountInfo(RPC_URL, owner.publicKey),
    contractHash = findKeyFromAccountNamedKeys(
      account,
      `cep18_contract_hash_${name}`
    );

  const cep18 = new CEP18Client(RPC_URL, SSE_URL, CHAIN_NAME)
    .setContractHash(contractHash)
    .startEventStream();

  console.info(`Contract Hash: ${cep18.contractHash.toPrefixedString()}`);

  const isMintAndBurnEnabled = await cep18.isMintAndBurnEnabled();

  if (!isMintAndBurnEnabled) {
    console.warn(`Mint and Burn is disabled.`);
    return;
  }

  // Mint tokens
  let params: TransactionParams = {
    sender: owner.publicKey,
    paymentAmount: String(5_000_000_000),
    signingKeys: [owner]
  };

  const mintArgs: MintArgs = {
    owner: ali.publicKey,
    amount: String(10_000_000_000)
  };

  await cep18.mint({
    params,
    args: mintArgs
  });

  const mintEvent = CEP18_EVENTS.Mint;
  await new Promise<void>((resolve, reject) => {
    cep18.on(mintEvent, async eventResult => {
      try {
        await eventListener(cep18, mintEvent, eventResult);
        resolve();
      } catch (error) {
        reject(error);
      }
    });
  });

  const aliBalance = await cep18.balanceOf(ali.publicKey);
  console.info(
    `Token minted successfully, Ali's balance: ${aliBalance.toString()}`
  );

  // Burn tokens
  params = {
    sender: ali.publicKey,
    paymentAmount: String(5_000_000_000),
    signingKeys: [ali]
  };

  const burnArgs: BurnArgs = {
    owner: ali.publicKey,
    amount: String(10_000_000_000)
  };

  await cep18.burn({
    params,
    args: burnArgs
  });

  const burnEvent = CEP18_EVENTS.Burn;
  await new Promise<void>((resolve, reject) => {
    cep18.on(burnEvent, async eventResult => {
      try {
        await eventListener(cep18, burnEvent, eventResult);
        resolve();
      } catch (error) {
        reject(error);
      }
    });
  });

  const newBalance = await cep18.balanceOf(ali.publicKey);
  console.info(
    `Token burnt successfully, Ali's balance: ${newBalance.toString()}`
  );

  cep18.stopEventStream();

  // Change Events mode to Native events, wait for transaction instead of listener
  params = {
    sender: owner.publicKey,
    paymentAmount: String(3_000_000_000),
    signingKeys: [owner]
  };

  const changeEventsModeArgs = {
    eventsMode: EVENTS_MODE.Native
  };

  await cep18.changeEventsMode({
    params,
    args: changeEventsModeArgs,
    waitForTransactionProcessed: true
  });

  const newEventsMode = await cep18.eventsMode();
  console.info(
    `Events Mode changed successfully, new events mode : ${newEventsMode.toString()}`
  );
};

const eventListener = async (
  cep18: CEP18Client,
  eventType: keyof typeof CEP18_EVENTS,
  eventResult: CEP18EventResult
) => {
  const { transactionInfo, executionResult } = await cep18
    .getTransactionResult(eventResult.transactionInfo.transactionHash.toHex())
    .then((transactionResult: InfoGetTransactionResult) => ({
      transactionInfo: eventResult.transactionInfo,
      executionResult: transactionResult.executionInfo?.executionResult
    }));

  console.info(
    `Contract ${eventType} transaction hash: ${transactionInfo.transactionHash.toHex()}`
  );

  if (executionResult) {
    if (executionResult?.errorMessage) {
      throw new Error(
        `Error during ${eventType}.\n${executionResult?.errorMessage.toString()}`
      );
    } else {
      console.info(
        `Contract ${eventType} cost consumed: ${executionResult?.consumed}`
      );
    }
  }
  cep18.removeListenersForEvent(CEP18_EVENTS[eventType]);
};

usage()
  .then(() => {
    console.info('Events usage completed.');
  })
  .catch(error => {
    console.error('Usage failed:', error);
  });

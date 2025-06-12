import {
  CLValue,
  EventSubscription,
  Hash,
  InfoGetTransactionResult,
  Key,
  SseClient,
  TransactionHash,
  TransactionProcessedEvent
} from 'casper-js-sdk';
import { describe, it, expect, vi, beforeEach } from 'vitest';
import Client from '../../src/client';
import {
  CEP18_EVENTS,
  Event,
  Mint,
  WithTransactionInfo
} from '../../src/events';

const mockTransactionHash = { toHex: () => 'mockTransactionHash' };

describe('Client Class', () => {
  let client: Client;

  beforeEach(() => {
    client = new Client(
      'http://mock-rpc-url',
      'http://mock-sse-url',
      'testnet'
    );
  });

  it('should initialize with given URLs and chain name', () => {
    expect(client.chainName).toBe('testnet');
  });

  it('should set RPC URL correctly', () => {
    client.rpcUrl = 'http://new-rpc-url';
    expect(client.rpcUrl).toBe('http://new-rpc-url');
  });

  it('should set SSE URL correctly', () => {
    client.sseUrl = 'http://new-sse-url';
    expect(client.sseUrl).toBe('http://new-sse-url');
  });

  it('should add an event listener', () => {
    const mockListener = vi.fn();
    client.addEventListener('testEvent', mockListener);
    expect(client['_events']['testEvent']).toContain(mockListener);
  });

  it('should remove an event listener', () => {
    const mockListener = vi.fn();
    client.addEventListener('testEvent', mockListener);
    client.removeEventListener('testEvent', mockListener);
    expect(client['_events']['testEvent']).not.toContain(mockListener);
  });

  it('should throw an error when removing a non-existent event listener', () => {
    const mockListener = vi.fn();
    expect(() =>
      client.removeEventListener('nonExistentEvent', mockListener)
    ).toThrow(
      'Can\'t remove a listener. Event "nonExistentEvent" doesn\'t exist.'
    );
  });

  it('should remove all listeners for a specific event', () => {
    const mockListener1 = vi.fn();
    const mockListener2 = vi.fn();

    client.addEventListener('testEvent', mockListener1);
    client.addEventListener('testEvent', mockListener2);

    client.removeListenersForEvent('testEvent');
    expect(client['_events']['testEvent']).toEqual([]);
  });

  it('should throw an error when removing listeners for a non-existent event', () => {
    expect(() => client.removeListenersForEvent('nonExistentEvent')).toThrow(
      'No listeners found for event "nonExistentEvent".'
    );
  });

  it('should remove all event listeners', () => {
    const mockListener1 = vi.fn();
    const mockListener2 = vi.fn();

    client.addEventListener('event1', mockListener1);
    client.addEventListener('event2', mockListener2);

    client.removeAllListeners();
    expect(client['_events']['event1']).toEqual([]);
    expect(client['_events']['event2']).toEqual([]);
  });

  it('should call event listeners when an event occurs', () => {
    const mockListener = vi.fn();
    client.addEventListener('testEvent', mockListener);

    const mockEvent: WithTransactionInfo<Event<Mint>> = {
      transactionInfo: {
        transactionHash: mockTransactionHash as TransactionHash,
        timestamp: 'timestamp',
        messages: []
      },
      name: CEP18_EVENTS.Mint,
      contractHash: new Hash(new Uint8Array(32)),
      contractPackageHash: new Hash(new Uint8Array(32)),
      eventId: 1,
      data: {
        recipient: CLValue.newCLKey(
          Key.newKey(
            'account-hash-1470f2fe74dee714d7075015e928b837afcfc689c3e0c40c84dd041c7fa1fd0d'
          )
        ),
        amount: CLValue.newCLString('')
      }
    };

    // Simulate an event trigger
    client['_events']['testEvent'].forEach(listener => listener(mockEvent));

    expect(mockListener).toHaveBeenCalledWith(mockEvent);
  });

  it("should use 'on' alias to add an event listener", () => {
    const mockListener = vi.fn();
    client.on('aliasEvent', mockListener);
    expect(client['_events']['aliasEvent']).toContain(mockListener);
  });

  it("should use 'off' alias to remove an event listener", () => {
    const mockListener = vi.fn();
    client.on('aliasEvent', mockListener);
    client.off('aliasEvent', mockListener);
    expect(client['_events']['aliasEvent']).not.toContain(mockListener);
  });

  describe('Client - getTransactionResult', () => {
    it('should handle both successful and failed transaction results', async () => {
      let client = new Client('http://mock-rpc-url');
      const mockSuccessResult = {
        executionInfo: { executionResult: { errorMessage: null } }
      } as unknown as InfoGetTransactionResult;

      const errorMessage = 'Transaction failed';
      const mockErrorResult = {
        executionInfo: {
          executionResult: { errorMessage }
        }
      } as unknown as InfoGetTransactionResult;

      const consoleErrorSpy = vi
        .spyOn(console, 'error')
        .mockImplementation(() => {});

      // Mocking the RpcClient's `getTransactionByTransactionHash` method for the rpcClient
      vi.spyOn(client['_rpcClient'], 'getTransactionByTransactionHash')
        .mockResolvedValueOnce(mockSuccessResult) // First mock: Success
        .mockResolvedValueOnce(mockErrorResult); // Second mock: Failure

      // Test success case
      await expect(client.getTransactionResult('mockHash1')).resolves.toEqual(
        mockSuccessResult
      );

      // Test failure case
      await expect(client.getTransactionResult('mockHash2')).resolves.toEqual(
        mockErrorResult
      );
      expect(consoleErrorSpy).toHaveBeenCalledWith(`Error: ${errorMessage}`);
    });
  });

  describe('Client - waitForTransactionProcessed', () => {
    let client: Client;
    let mockSseClient: SseClient;

    beforeEach(() => {
      // Mocking the SseClient's methods
      mockSseClient = {
        start: vi.fn(),
        stop: vi.fn(),
        unsubscribe: vi.fn()
      } as unknown as SseClient;

      client = new Client('http://mock-rpc-url');
      client['_sseClient'] = mockSseClient;
    });

    it('should resolve when transaction is processed', async () => {
      const mockProcessedEvent: TransactionProcessedEvent = {
        transactionProcessedPayload: {
          transactionHash: mockTransactionHash as TransactionHash
        }
      } as unknown as TransactionProcessedEvent;

      // Mock the subscription event behavior
      vi.spyOn(
        client as any,
        'subscribeToTransactionProcessedEvent'
      ).mockImplementation((callback: any) => {
        // Simulate processing of the event
        setTimeout(() => {
          callback(mockProcessedEvent); // Invoke the callback with mock event
        }, 100);
        return {} as unknown as EventSubscription; // Return the mock subscription
      });

      // Test the successful case where the transaction gets processed
      await expect(
        client.waitForTransactionProcessed(mockTransactionHash.toHex())
      ).resolves.toEqual(mockProcessedEvent);

      // Ensure that the SSE client methods were called
      expect(mockSseClient.start).toHaveBeenCalled();
      expect(mockSseClient.stop).toHaveBeenCalled();
      expect(mockSseClient.unsubscribe).toHaveBeenCalled();
    });

    it('should use the provided sseUrl when calling waitForTransactionProcessed', async () => {
      const mockProcessedEvent: TransactionProcessedEvent = {
        transactionProcessedPayload: {
          transactionHash: mockTransactionHash
        }
      } as unknown as TransactionProcessedEvent;

      const timeout = 200;
      const mockSseUrl = 'http://mock-sse-url'; // Provided sseUrl

      vi.spyOn(
        client as any,
        'subscribeToTransactionProcessedEvent'
      ).mockImplementation((callback: any) => {
        // Simulate processing of the event
        setTimeout(() => {
          callback(mockProcessedEvent); // Invoke the callback with mock event
        }, 100);
        return {} as unknown as EventSubscription; // Return the mock subscription
      });

      vi.spyOn(client, 'sseUrl', 'set').mockImplementation(() => {
        client['_sseClient'] = mockSseClient;
        client['_sseUrl'] = mockSseUrl;
      });

      // Test the case where sseUrl is passed to the method
      await expect(
        client.waitForTransactionProcessed(
          mockTransactionHash.toHex(),
          timeout,
          mockSseUrl
        )
      ).resolves.toEqual(mockProcessedEvent);

      expect(mockSseClient.start).toHaveBeenCalled();
      expect(mockSseClient.stop).toHaveBeenCalled();
      expect(mockSseClient.unsubscribe).toHaveBeenCalled();

      expect(client['_sseUrl']).toBe(mockSseUrl);
    });

    it('should reject if transaction processing times out', async () => {
      const timeout = 200; // Mock timeout duration

      // Mock the subscription to simulate no event processing
      vi.spyOn(
        client as any,
        'subscribeToTransactionProcessedEvent'
      ).mockImplementation(() => {
        // Simulate no event trigger
        return {} as unknown as any;
      });

      // Test the timeout scenario
      await expect(
        client.waitForTransactionProcessed(mockTransactionHash.toHex(), timeout)
      ).rejects.toThrow(
        `Transaction ${mockTransactionHash.toHex()} processing timed out.`
      );

      // Check if SSE client methods were called
      expect(mockSseClient.start).toHaveBeenCalled();
      expect(mockSseClient.stop).toHaveBeenCalled();
    });

    it('should reject if SSE Client is not set', async () => {
      client['_sseClient'] = undefined as any; // Make sure the SSE client is not set

      await expect(
        client.waitForTransactionProcessed(mockTransactionHash.toHex())
      ).rejects.toThrow();
    });

    it('should reject if an error occurs during event processing', async () => {
      const mockError = new Error('Mock error during processing');

      // Mock the subscription to simulate an error
      vi.spyOn(
        client as any,
        'subscribeToTransactionProcessedEvent'
      ).mockImplementation((_, errorCallback: any) => {
        // Simulate an error
        setTimeout(() => {
          errorCallback(mockError);
        }, 100);
        return {} as unknown as any;
      });

      // Test the error scenario
      await expect(
        client.waitForTransactionProcessed(mockTransactionHash.toHex())
      ).rejects.toThrow(mockError);

      // Check if SSE client methods were called
      expect(mockSseClient.start).toHaveBeenCalled();
      expect(mockSseClient.stop).toHaveBeenCalled();
    });
  });
});

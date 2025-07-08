import {
  Args,
  CLTypeKey,
  CLValue,
  ContractHash,
  ContractPackageHash,
  ExecutionResult,
  Key,
  KeyAlgorithm,
  PrivateKey,
  PublicKey,
  PutTransactionResult,
  RpcClient,
  StateGetDictionaryResult,
  TransactionProcessedPayload
} from 'casper-js-sdk';
import { describe, it, expect, vi, beforeEach, afterEach } from 'vitest';
import {
  EVENTS_MODE,
  InstallParams,
  CEP18Client,
  UpgradeParams,
  TransferParams,
  TransferFromParams,
  ApproveParams,
  DecreaseAllowanceParams,
  MintParams,
  BurnParams,
  ChangeSecurityParams,
  ChangeEventsModeParams
} from '../../src';

const mockTransactionHash = { toHex: () => 'mockTransactionHash' };

describe('CEP18Client Unit', () => {
  describe('CEP18Client - setContractHash', () => {
    let client: CEP18Client;
    beforeEach(() => {
      // Initializing a new CEP18Client instance for each test
      client = new CEP18Client('http://mock-rpc-url');
    });

    afterEach(() => {
      vi.restoreAllMocks();
    });

    it('should correctly set the contract hash and contract package hash', () => {
      const contractHash = 'contract-hash-0x';
      const contractPackageHash = 'contract-package-0x';
      // Spy on the method to see if the contract hash is set properly
      vi.spyOn(ContractHash, 'newContract').mockImplementation(() => {
        return {} as ContractHash;
      });
      vi.spyOn(ContractPackageHash, 'newContractPackage').mockImplementation(
        () => {
          return {} as ContractPackageHash;
        }
      );
      const result = client.setContractHash(contractHash, contractPackageHash);
      // Check if the correct methods were called for both contract hash and contract package hash
      expect(ContractHash.newContract).toHaveBeenCalledWith('0x');
      expect(ContractPackageHash.newContractPackage).toHaveBeenCalledWith('0x');
      expect(result).toBeInstanceOf(CEP18Client);
    });

    it('should throw an error if contract hash is not provided', () => {
      // Providing invalid contract hash
      expect(() => client.setContractHash('')).toThrowError(
        'Contract hash must be provided.'
      );
    });

    it('should correctly remove prefixes from the contract hash and contract package hash', () => {
      const contractHashWithPrefix = 'hash-12345';
      const contractPackageHashWithPrefix = 'package-0x';
      // Mock implementation of `ContractHash` and `ContractPackageHash`
      vi.spyOn(ContractHash, 'newContract').mockImplementation(() => {
        return {} as ContractHash;
      });
      vi.spyOn(ContractPackageHash, 'newContractPackage').mockImplementation(
        () => {
          return {} as ContractPackageHash;
        }
      );
      client.setContractHash(
        contractHashWithPrefix,
        contractPackageHashWithPrefix
      );
      // Ensure the prefixes are correctly removed
      expect(ContractHash.newContract).toHaveBeenCalledWith('12345');
      expect(ContractPackageHash.newContractPackage).toHaveBeenCalledWith('0x');
    });

    it('should handle optional contract package hash', () => {
      const contractHash = 'contract-hash-0x';
      // Mock implementation of `ContractHash`
      vi.spyOn(ContractHash, 'newContract').mockImplementation(() => {
        return {} as ContractHash;
      });
      // No contract package hash provided, so it should still work
      const result = client.setContractHash(contractHash);
      expect(ContractHash.newContract).toHaveBeenCalledWith('0x');
      expect(result).toBeInstanceOf(CEP18Client);
    });
  });

  describe('CEP18Client - Event Stream', () => {
    let client: CEP18Client;
    let mockSseUrl: string;

    beforeEach(() => {
      client = new CEP18Client(
        'http://mock-rpc-url',
        'http://mock-sse-url',
        'testnet'
      );
      mockSseUrl = 'http://mock-sse-url';
    });

    it('should call startEventStream and return the updated CEP18Client instance', () => {
      // Spy on the super class method
      const startEventStreamSpy = vi
        .spyOn(CEP18Client.prototype, 'startEventStream')
        .mockReturnThis();

      const result = client.startEventStream(mockSseUrl);
      expect(startEventStreamSpy).toHaveBeenCalledWith(mockSseUrl);
      expect(result).toBe(client);
    });

    it('should call stopEventStream and return the updated CEP18Client instance', () => {
      // Spy on the super class method
      const stopEventStreamSpy = vi
        .spyOn(CEP18Client.prototype, 'stopEventStream')
        .mockReturnThis();

      // Call stopEventStream and check that it works
      const result = client.stopEventStream();
      expect(stopEventStreamSpy).toHaveBeenCalled();
      expect(result).toBe(client); // Expecting the same instance to be returned
    });

    it('should handle undefined sseUrl gracefully', () => {
      const clientWithUndefinedSseUrl = new CEP18Client(
        'http://mock-rpc-url',
        undefined, // undefined sseUrl for testing
        'testnet'
      );

      const startEventStreamSpy = vi
        .spyOn(CEP18Client.prototype, 'startEventStream')
        .mockReturnThis();

      const result = clientWithUndefinedSseUrl.startEventStream(
        'http://mock-sse-url'
      );
      expect(startEventStreamSpy).toHaveBeenCalledWith(mockSseUrl);
      expect(result).toBe(clientWithUndefinedSseUrl);
    });
  });

  describe('CEP18Client - install', () => {
    let client: CEP18Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const mockParams: InstallParams = {
      params: {
        wasm: new Uint8Array(),
        paymentAmount: '1000',
        sender: key.publicKey,
        chainName: 'testnet',
        signingKeys: [key]
      },
      args: {
        name: 'CEP18',
        symbol: 'CEP18',
        decimals: 18,
        totalSupply: String(1000000),
        eventsMode: EVENTS_MODE.CES,
        enableMintAndBurn: true
      },
      waitForTransactionProcessed: false
    };

    beforeEach(() => {
      client = new CEP18Client('http://mock-rpc-url');
      vi.spyOn(client['_rpcClient'], 'putTransaction').mockResolvedValue({
        transactionHash: mockTransactionHash
      } as unknown as PutTransactionResult);
      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult
        } as unknown as TransactionProcessedPayload
      });
    });

    it('should successfully install a contract', async () => {
      const result = await client.install(mockParams);

      expect(client['_rpcClient'].putTransaction).toHaveBeenCalled();
      expect(result.transactionInfo.transactionHash.toHex()).toBe(
        'mockTransactionHash'
      );
    });

    it('should call waitForTransactionProcessed if waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };

      await client.install(paramsWithWait);

      expect(client.waitForTransactionProcessed).toHaveBeenCalledWith(
        'mockTransactionHash'
      );
    });

    it('should successfully install a contract if waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      const result = await client.install(paramsWithWait);

      expect(client['_rpcClient'].putTransaction).toHaveBeenCalled();

      expect(result.transactionInfo.transactionHash.toHex()).toBe(
        'mockTransactionHash'
      );
      expect(result.executionResult?.errorMessage).toBe('');
    });

    it('should handle errors during transaction installation', async () => {
      const errorMessage = 'error during installation';
      vi.spyOn(client['_rpcClient'], 'putTransaction').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.install(mockParams)).rejects.toThrow(
        `Error during installation runtime.\nError: ${errorMessage}`
      );
    });
  });

  describe('CEP18Client - upgrade', () => {
    let client: CEP18Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const mockParams: UpgradeParams = {
      params: {
        wasm: new Uint8Array(),
        paymentAmount: '1000',
        sender: key.publicKey,
        chainName: 'testnet',
        signingKeys: [key]
      },
      args: {
        name: 'CEP18',
        eventsMode: EVENTS_MODE.CES
      },
      waitForTransactionProcessed: false
    };

    beforeEach(() => {
      client = new CEP18Client('http://mock-rpc-url');
      vi.spyOn(client['_rpcClient'], 'putTransaction').mockResolvedValue({
        transactionHash: mockTransactionHash
      } as unknown as PutTransactionResult);
      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult
        } as unknown as TransactionProcessedPayload
      });
    });

    it('should successfully upgrade a contract', async () => {
      const result = await client.upgrade(mockParams);

      expect(client['_rpcClient'].putTransaction).toHaveBeenCalled();
      expect(result.transactionInfo.transactionHash.toHex()).toBe(
        'mockTransactionHash'
      );
    });

    it('should call waitForTransactionProcessed if waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };

      await client.upgrade(paramsWithWait);

      expect(client.waitForTransactionProcessed).toHaveBeenCalledWith(
        'mockTransactionHash'
      );
    });

    it('should successfully upgrade a contract if waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      const result = await client.upgrade(paramsWithWait);

      expect(client['_rpcClient'].putTransaction).toHaveBeenCalled();

      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });
    });

    it('should handle errors during transaction upgradeation', async () => {
      const errorMessage = 'error during upgrade';
      vi.spyOn(client['_rpcClient'], 'putTransaction').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.upgrade(mockParams)).rejects.toThrow(
        `Error during upgrade runtime.\nError: ${errorMessage}`
      );
    });
  });

  describe('CEP18Client - transfer', () => {
    let client: CEP18Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const mockParams: TransferParams = {
      params: {
        sender: key.publicKey,
        paymentAmount: '1000',
        signingKeys: [key],
        chainName: 'testnet'
      },
      args: {
        recipient: key.publicKey,
        amount: '1000'
      },
      waitForTransactionProcessed: false
    };

    beforeEach(() => {
      client = new CEP18Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult
        } as unknown as TransactionProcessedPayload
      });
    });

    it('should successfully transfer', async () => {
      const result = await client.transfer(mockParams);

      // Check that callEntrypoint was called with correct parameters
      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'transfer',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        mockParams.waitForTransactionProcessed
      );

      // Check the result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
    });

    it('should call callEntrypoint with correct runtime arguments', async () => {
      await client.transfer(mockParams);

      const runtimeArgs = (client as any).callEntrypoint.mock.calls[0][1];

      // Assert the runtime arguments for transfer
      expect(runtimeArgs).toEqual(
        Args.fromMap({
          recipient: CLValue.newCLKey(
            Key.newKey(key.publicKey.accountHash().toPrefixedString())
          ),
          amount: CLValue.newCLUInt256(mockParams.args.amount)
        })
      );
    });

    it('should successfully transfer when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });

      const result = await client.transfer(paramsWithWait);

      // Check the result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });
    });

    it('should handle the case when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      await client.transfer(paramsWithWait);

      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'transfer',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        true
      );
    });

    it('should handle errors during the transfer process', async () => {
      const errorMessage = 'Error during transfer.';
      vi.spyOn(client as any, 'callEntrypoint').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.transfer(mockParams)).rejects.toThrow(
        'Error during transfer.'
      );
    });
  });

  describe('CEP18Client - transferFrom', () => {
    let client: CEP18Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const ownerKey = PrivateKey.generate(KeyAlgorithm.ED25519);
    const mockParams: TransferFromParams = {
      params: {
        sender: key.publicKey,
        paymentAmount: '1000',
        signingKeys: [key],
        chainName: 'testnet'
      },
      args: {
        owner: ownerKey.publicKey,
        recipient: key.publicKey,
        amount: '1000'
      },
      waitForTransactionProcessed: false
    };

    beforeEach(() => {
      client = new CEP18Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult
        } as unknown as TransactionProcessedPayload
      });
    });

    it('should successfully execute transferFrom', async () => {
      const result = await client.transferFrom(mockParams);

      // Verify callEntrypoint was called with correct parameters
      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'transfer_from',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        mockParams.waitForTransactionProcessed
      );

      // Verify result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
    });

    it('should call callEntrypoint with correct runtime arguments', async () => {
      await client.transferFrom(mockParams);

      // Retrieve runtimeArgs from spy call
      const runtimeArgs = (client as any).callEntrypoint.mock.calls[0][1];

      // Validate runtime arguments for transferFrom
      expect(runtimeArgs).toEqual(
        Args.fromMap({
          owner: CLValue.newCLKey(
            Key.newKey(ownerKey.publicKey.accountHash().toPrefixedString())
          ),
          recipient: CLValue.newCLKey(
            Key.newKey(key.publicKey.accountHash().toPrefixedString())
          ),
          amount: CLValue.newCLUInt256(mockParams.args.amount)
        })
      );
    });

    it('should successfully execute transferFrom when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });

      const result = await client.transferFrom(paramsWithWait);

      // Validate result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });
    });

    it('should handle the case when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      await client.transferFrom(paramsWithWait);

      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'transfer_from',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        true
      );
    });

    it('should handle errors during the transferFrom process', async () => {
      const errorMessage = 'Error during transferFrom.';
      vi.spyOn(client as any, 'callEntrypoint').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.transferFrom(mockParams)).rejects.toThrow(
        'Error during transferFrom.'
      );
    });
  });

  describe('CEP18Client - approve', () => {
    let client: CEP18Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const spenderKey = PrivateKey.generate(KeyAlgorithm.ED25519);
    const mockParams: ApproveParams = {
      params: {
        sender: key.publicKey,
        paymentAmount: '1000',
        signingKeys: [key],
        chainName: 'testnet'
      },
      args: {
        spender: spenderKey.publicKey,
        amount: '5000'
      },
      waitForTransactionProcessed: false
    };

    beforeEach(() => {
      client = new CEP18Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult
        } as unknown as TransactionProcessedPayload
      });
    });

    it('should successfully execute approve', async () => {
      const result = await client.approve(mockParams);

      // Verify callEntrypoint was called with correct parameters
      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'approve',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        mockParams.waitForTransactionProcessed
      );

      // Validate result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
    });

    it('should call callEntrypoint with correct runtime arguments', async () => {
      await client.approve(mockParams);

      // Retrieve runtimeArgs from spy call
      const runtimeArgs = (client as any).callEntrypoint.mock.calls[0][1];

      // Validate runtime arguments for approve
      expect(runtimeArgs).toEqual(
        Args.fromMap({
          spender: CLValue.newCLKey(
            Key.newKey(spenderKey.publicKey.accountHash().toPrefixedString())
          ),
          amount: CLValue.newCLUInt256(mockParams.args.amount)
        })
      );
    });

    it('should successfully execute approve when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });

      const result = await client.approve(paramsWithWait);

      // Validate result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });
    });

    it('should handle the case when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      await client.approve(paramsWithWait);

      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'approve',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        true
      );
    });

    it('should handle errors during the approve process', async () => {
      const errorMessage = 'Error during approve.';
      vi.spyOn(client as any, 'callEntrypoint').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.approve(mockParams)).rejects.toThrow(
        'Error during approve.'
      );
    });
  });

  describe('CEP18Client - increaseAllowance', () => {
    let client: CEP18Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const spenderKey = PrivateKey.generate(KeyAlgorithm.ED25519);
    const mockParams: ApproveParams = {
      params: {
        sender: key.publicKey,
        paymentAmount: '1000',
        signingKeys: [key],
        chainName: 'testnet'
      },
      args: {
        spender: spenderKey.publicKey,
        amount: '5000'
      },
      waitForTransactionProcessed: false
    };

    beforeEach(() => {
      client = new CEP18Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult
        } as unknown as TransactionProcessedPayload
      });
    });

    it('should successfully execute increaseAllowance', async () => {
      const result = await client.increaseAllowance(mockParams);

      // Verify callEntrypoint was called with correct parameters
      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'increase_allowance',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        mockParams.waitForTransactionProcessed
      );

      // Validate result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
    });

    it('should call callEntrypoint with correct runtime arguments', async () => {
      await client.increaseAllowance(mockParams);

      // Retrieve runtimeArgs from spy call
      const runtimeArgs = (client as any).callEntrypoint.mock.calls[0][1];

      // Validate runtime arguments for increaseAllowance
      expect(runtimeArgs).toEqual(
        Args.fromMap({
          spender: CLValue.newCLKey(
            Key.newKey(spenderKey.publicKey.accountHash().toPrefixedString())
          ),
          amount: CLValue.newCLUInt256(mockParams.args.amount)
        })
      );
    });

    it('should successfully execute increaseAllowance when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });

      const result = await client.increaseAllowance(paramsWithWait);

      // Validate result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });
    });

    it('should handle the case when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      await client.increaseAllowance(paramsWithWait);

      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'increase_allowance',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        true
      );
    });

    it('should handle errors during the increaseAllowance process', async () => {
      const errorMessage = 'Error during increaseAllowance.';
      vi.spyOn(client as any, 'callEntrypoint').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.increaseAllowance(mockParams)).rejects.toThrow(
        'Error during increaseAllowance.'
      );
    });
  });

  describe('CEP18Client - decreaseAllowance', () => {
    let client: CEP18Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const spenderKey = PrivateKey.generate(KeyAlgorithm.ED25519);
    const mockParams: DecreaseAllowanceParams = {
      params: {
        sender: key.publicKey,
        paymentAmount: '1000',
        signingKeys: [key],
        chainName: 'testnet'
      },
      args: {
        spender: spenderKey.publicKey,
        amount: '2000'
      },
      waitForTransactionProcessed: false
    };

    beforeEach(() => {
      client = new CEP18Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult
        } as unknown as TransactionProcessedPayload
      });
    });

    it('should successfully execute decreaseAllowance', async () => {
      const result = await client.decreaseAllowance(mockParams);

      // Verify callEntrypoint was called with correct parameters
      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'decrease_allowance',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        mockParams.waitForTransactionProcessed
      );

      // Validate result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
    });

    it('should call callEntrypoint with correct runtime arguments', async () => {
      await client.decreaseAllowance(mockParams);

      // Retrieve runtimeArgs from spy call
      const runtimeArgs = (client as any).callEntrypoint.mock.calls[0][1];

      // Validate runtime arguments for decreaseAllowance
      expect(runtimeArgs).toEqual(
        Args.fromMap({
          spender: CLValue.newCLKey(
            Key.newKey(spenderKey.publicKey.accountHash().toPrefixedString())
          ),
          amount: CLValue.newCLUInt256(mockParams.args.amount)
        })
      );
    });

    it('should successfully execute decreaseAllowance when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });

      const result = await client.decreaseAllowance(paramsWithWait);

      // Validate result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });
    });

    it('should handle the case when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      await client.decreaseAllowance(paramsWithWait);

      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'decrease_allowance',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        true
      );
    });

    it('should handle errors during the decreaseAllowance process', async () => {
      const errorMessage = 'Error during decreaseAllowance.';
      vi.spyOn(client as any, 'callEntrypoint').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.decreaseAllowance(mockParams)).rejects.toThrow(
        'Error during decreaseAllowance.'
      );
    });
  });

  describe('CEP18Client - mint', () => {
    let client: CEP18Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const ownerKey = PrivateKey.generate(KeyAlgorithm.ED25519);
    const mockParams: MintParams = {
      params: {
        sender: key.publicKey,
        paymentAmount: '1000',
        signingKeys: [key],
        chainName: 'testnet'
      },
      args: {
        owner: ownerKey.publicKey,
        amount: '5000'
      },
      waitForTransactionProcessed: false
    };

    beforeEach(() => {
      client = new CEP18Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult
        } as unknown as TransactionProcessedPayload
      });
    });

    it('should successfully execute mint', async () => {
      const result = await client.mint(mockParams);

      // Verify callEntrypoint was called with correct parameters
      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'mint',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        mockParams.waitForTransactionProcessed
      );

      // Validate result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
    });

    it('should call callEntrypoint with correct runtime arguments', async () => {
      await client.mint(mockParams);

      // Retrieve runtimeArgs from spy call
      const runtimeArgs = (client as any).callEntrypoint.mock.calls[0][1];

      // Validate runtime arguments for mint
      expect(runtimeArgs).toEqual(
        Args.fromMap({
          owner: CLValue.newCLKey(
            Key.newKey(ownerKey.publicKey.accountHash().toPrefixedString())
          ),
          amount: CLValue.newCLUInt256(mockParams.args.amount)
        })
      );
    });

    it('should successfully execute mint when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });

      const result = await client.mint(paramsWithWait);

      // Validate result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });
    });

    it('should handle the case when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      await client.mint(paramsWithWait);

      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'mint',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        true
      );
    });

    it('should handle errors during the mint process', async () => {
      const errorMessage = 'Error during mint.';
      vi.spyOn(client as any, 'callEntrypoint').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.mint(mockParams)).rejects.toThrow(
        'Error during mint.'
      );
    });
  });

  describe('CEP18Client - burn', () => {
    let client: CEP18Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const ownerKey = PrivateKey.generate(KeyAlgorithm.ED25519);
    const mockParams: BurnParams = {
      params: {
        sender: key.publicKey,
        paymentAmount: '1000',
        signingKeys: [key],
        chainName: 'testnet'
      },
      args: {
        owner: ownerKey.publicKey,
        amount: '3000'
      },
      waitForTransactionProcessed: false
    };

    beforeEach(() => {
      client = new CEP18Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult
        } as unknown as TransactionProcessedPayload
      });
    });

    it('should successfully execute burn', async () => {
      const result = await client.burn(mockParams);

      // Verify callEntrypoint was called with correct parameters
      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'burn',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        mockParams.waitForTransactionProcessed
      );

      // Validate result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
    });

    it('should call callEntrypoint with correct runtime arguments', async () => {
      await client.burn(mockParams);

      // Retrieve runtimeArgs from spy call
      const runtimeArgs = (client as any).callEntrypoint.mock.calls[0][1];

      // Validate runtime arguments for burn
      expect(runtimeArgs).toEqual(
        Args.fromMap({
          owner: CLValue.newCLKey(
            Key.newKey(ownerKey.publicKey.accountHash().toPrefixedString())
          ),
          amount: CLValue.newCLUInt256(mockParams.args.amount)
        })
      );
    });

    it('should successfully execute burn when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });

      const result = await client.burn(paramsWithWait);

      // Validate result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });
    });

    it('should handle the case when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      await client.burn(paramsWithWait);

      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'burn',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        true
      );
    });

    it('should handle errors during the burn process', async () => {
      const errorMessage = 'Error during burn.';
      vi.spyOn(client as any, 'callEntrypoint').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.burn(mockParams)).rejects.toThrow(
        'Error during burn.'
      );
    });
  });

  describe('CEP18Client - changeSecurity', () => {
    let client: CEP18Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const adminKey = PrivateKey.generate(KeyAlgorithm.ED25519);
    const minterKey = PrivateKey.generate(KeyAlgorithm.ED25519);
    const noneKey = PrivateKey.generate(KeyAlgorithm.ED25519);

    const mockParams: ChangeSecurityParams = {
      params: {
        sender: key.publicKey,
        paymentAmount: '1000',
        signingKeys: [key],
        chainName: 'testnet'
      },
      args: {
        adminList: [adminKey.publicKey],
        minterList: [minterKey.publicKey],
        noneList: [noneKey.publicKey]
      },
      waitForTransactionProcessed: false
    };

    beforeEach(() => {
      client = new CEP18Client('http://mock-rpc-url');
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult
        } as unknown as TransactionProcessedPayload
      });
    });

    it('should successfully execute changeSecurity', async () => {
      const result = await client.changeSecurity(mockParams);

      // Verify callEntrypoint was called with correct parameters
      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'change_security',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        mockParams.waitForTransactionProcessed
      );

      // Validate result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
    });

    it('should call callEntrypoint with correct runtime arguments', async () => {
      await client.changeSecurity(mockParams);

      // Retrieve runtimeArgs from spy call
      const runtimeArgs = (client as any).callEntrypoint.mock.calls[0][1];

      // Validate runtime arguments for changeSecurity
      expect(runtimeArgs).toEqual(
        Args.fromMap({
          admin_list: CLValue.newCLList(
            CLTypeKey,
            mockParams.args.adminList?.map(key =>
              CLValue.newCLKey(
                Key.newKey((key as PublicKey).accountHash().toPrefixedString())
              )
            )
          ),
          minter_list: CLValue.newCLList(
            CLTypeKey,
            mockParams.args.minterList?.map(key =>
              CLValue.newCLKey(
                Key.newKey((key as PublicKey).accountHash().toPrefixedString())
              )
            )
          ),
          none_list: CLValue.newCLList(
            CLTypeKey,
            mockParams.args.noneList?.map(key =>
              CLValue.newCLKey(
                Key.newKey((key as PublicKey).accountHash().toPrefixedString())
              )
            )
          )
        })
      );
    });

    it('should successfully execute changeSecurity when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });

      const result = await client.changeSecurity(paramsWithWait);

      // Validate result
      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });
    });

    it('should handle the case when waitForTransactionProcessed is true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };
      await client.changeSecurity(paramsWithWait);

      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'change_security',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        true
      );
    });

    it('should throw an error when no arguments are provided', async () => {
      const emptyArgsParams: ChangeSecurityParams = {
        ...mockParams,
        args: {} // Empty args
      };

      try {
        await client.changeSecurity(emptyArgsParams);
        expect(false).toBe(true); // Forces failure if no error is thrown
      } catch (error) {
        expect(error).toBeInstanceOf(Error);
        expect((error as Error).message).toBe(
          'Should provide at least one arg'
        );
      }
    });

    it('should handle errors during the changeSecurity process', async () => {
      const errorMessage = 'Error during changeSecurity.';
      vi.spyOn(client as any, 'callEntrypoint').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.changeSecurity(mockParams)).rejects.toThrow(
        'Error during changeSecurity.'
      );
    });
  });

  describe('CEP18Client - changeEventsMode', () => {
    let client: CEP18Client;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const eventsMode = EVENTS_MODE.Native;

    const mockParams: ChangeEventsModeParams = {
      params: {
        sender: key.publicKey,
        paymentAmount: '1000',
        signingKeys: [key],
        chainName: 'testnet'
      },
      args: { eventsMode },
      waitForTransactionProcessed: false
    };

    beforeEach(() => {
      client = new CEP18Client('http://mock-rpc-url');

      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });

      vi.spyOn(client, 'waitForTransactionProcessed').mockResolvedValue({
        transactionProcessedPayload: {
          executionResult: { errorMessage: '' } as ExecutionResult
        } as unknown as TransactionProcessedPayload
      });
    });

    it('should successfully execute changeEventsMode', async () => {
      const result = await client.changeEventsMode(mockParams);

      expect(client['callEntrypoint']).toHaveBeenCalledWith(
        'change_events_mode',
        expect.anything(),
        mockParams.params.paymentAmount,
        mockParams.params.sender,
        mockParams.params.signingKeys,
        mockParams.params.chainName,
        mockParams.waitForTransactionProcessed
      );

      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        }
      });
    });

    it('should call callEntrypoint with correct runtime arguments', async () => {
      await client.changeEventsMode(mockParams);

      const runtimeArgs = (client as any).callEntrypoint.mock.calls[0][1];

      expect(runtimeArgs).toEqual(
        Args.fromMap({
          events_mode: CLValue.newCLUint8(eventsMode)
        })
      );
    });

    it('should successfully execute changeEventsMode with waitForTransactionProcessed = true', async () => {
      const paramsWithWait = {
        ...mockParams,
        waitForTransactionProcessed: true
      };

      vi.spyOn(client as any, 'callEntrypoint').mockResolvedValue({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });

      const result = await client.changeEventsMode(paramsWithWait);

      expect(result).toEqual({
        transactionInfo: {
          transactionHash: mockTransactionHash
        },
        executionResult: { errorMessage: '' } as ExecutionResult
      });
    });

    it('should propagate error if callEntrypoint fails', async () => {
      const errorMessage = 'Failed to change events mode.';
      vi.spyOn(client as any, 'callEntrypoint').mockRejectedValueOnce(
        new Error(errorMessage)
      );

      await expect(client.changeEventsMode(mockParams)).rejects.toThrow(
        errorMessage
      );
    });
  });

  describe('CEP18Client - balanceOf', () => {
    let client: CEP18Client;
    let mockRpcClient: RpcClient;
    const key = PrivateKey.generate(KeyAlgorithm.ED25519);
    const contractHash =
      'hash-a84b9f15e57097579cb651bc3eec5143972c8c9ea153bb26d07367f9d41a767b';
    const mockBalance = '1000';

    beforeEach(() => {
      mockRpcClient = {
        getDictionaryItemByIdentifier: vi.fn()
      } as unknown as RpcClient;

      client = new CEP18Client('http://mock-rpc-url');
      client.setContractHash(contractHash);
      client['_rpcClient'] = mockRpcClient;

      vi.spyOn(
        mockRpcClient,
        'getDictionaryItemByIdentifier'
      ).mockResolvedValue({
        storedValue: {
          clValue: CLValue.newCLUInt512(mockBalance)
        }
      } as StateGetDictionaryResult);
    });

    it('should return the balance as a string when found', async () => {
      const balance = await client.balanceOf(key.publicKey);
      expect(balance).toBe(mockBalance);
    });

    it('should return "0" when balance is not found', async () => {
      vi.spyOn(
        mockRpcClient,
        'getDictionaryItemByIdentifier'
      ).mockResolvedValue({
        storedValue: { clValue: undefined }
      } as StateGetDictionaryResult);

      const balance = await client.balanceOf(key.publicKey);
      expect(balance).toBe('0');
    });

    it('should log a warning and return "0" when query fails with a missing balance', async () => {
      const consoleWarnSpy = vi
        .spyOn(console, 'warn')
        .mockImplementation(() => {});
      vi.spyOn(
        mockRpcClient,
        'getDictionaryItemByIdentifier'
      ).mockRejectedValue(new Error('Error: Query failed'));

      const balance = await client.balanceOf(key.publicKey);
      expect(balance).toBe('0');
      expect(consoleWarnSpy).toHaveBeenCalledWith(
        `No balance found for ${key.publicKey.accountHash().toPrefixedString()}`
      );
    });

    it('should throw an error if the query fails due to another issue', async () => {
      vi.spyOn(
        mockRpcClient,
        'getDictionaryItemByIdentifier'
      ).mockRejectedValue(new Error('Unexpected RPC error'));

      await expect(client.balanceOf(key.publicKey)).rejects.toThrow(
        'Unexpected RPC error'
      );
    });

    it('should construct the correct dictionary key', async () => {
      await client.balanceOf(key.publicKey);

      const dictionaryIdentifier = (mockRpcClient as any)
        .getDictionaryItemByIdentifier.mock.calls[0][1];

      expect(dictionaryIdentifier.contractNamedKey).toEqual({
        key: contractHash,
        dictionaryName: 'balances',
        dictionaryItemKey: expect.any(String)
      });
    });

    it('should throw an error when contract hash is not set in balanceOf', async () => {
      const clientWithoutContractHash = new CEP18Client('http://mock-rpc-url');
      await expect(
        clientWithoutContractHash.balanceOf(key.publicKey)
      ).rejects.toThrow('Contract hash is not set.');
    });
  });

  describe('CEP18Client - allowances', () => {
    let client: CEP18Client;
    let mockRpcClient: RpcClient;
    const keyOwner = PrivateKey.generate(KeyAlgorithm.ED25519);
    const keySpender = PrivateKey.generate(KeyAlgorithm.ED25519);
    const contractHash =
      'hash-a84b9f15e57097579cb651bc3eec5143972c8c9ea153bb26d07367f9d41a767b';
    const mockAllowance = '500';

    beforeEach(() => {
      mockRpcClient = {
        getDictionaryItemByIdentifier: vi.fn()
      } as unknown as RpcClient;

      client = new CEP18Client('http://mock-rpc-url');
      client['_rpcClient'] = mockRpcClient;
      client.setContractHash(contractHash);

      vi.spyOn(
        mockRpcClient,
        'getDictionaryItemByIdentifier'
      ).mockResolvedValue({
        storedValue: {
          clValue: CLValue.newCLUInt512(mockAllowance)
        }
      } as StateGetDictionaryResult);
    });

    it('should return the allowance as a string when found', async () => {
      const allowance = await client.allowances(
        keyOwner.publicKey,
        keySpender.publicKey
      );
      expect(allowance).toBe(mockAllowance);
    });

    it('should return "0" when allowance is not found', async () => {
      vi.spyOn(
        mockRpcClient,
        'getDictionaryItemByIdentifier'
      ).mockResolvedValue({
        storedValue: { clValue: undefined }
      } as StateGetDictionaryResult);

      const allowance = await client.allowances(
        keyOwner.publicKey,
        keySpender.publicKey
      );
      expect(allowance).toBe('0');
    });

    it('should log a warning and return "0" when query fails with a missing allowance', async () => {
      const consoleWarnSpy = vi
        .spyOn(console, 'warn')
        .mockImplementation(() => {});
      vi.spyOn(
        mockRpcClient,
        'getDictionaryItemByIdentifier'
      ).mockRejectedValue(new Error('Error: Query failed'));

      const allowance = await client.allowances(
        keyOwner.publicKey,
        keySpender.publicKey
      );
      expect(allowance).toBe('0');
      expect(consoleWarnSpy).toHaveBeenCalledWith(
        `No allowances found for ${keyOwner.publicKey.accountHash().toPrefixedString()}`
      );
    });

    it('should throw an error if the query fails due to another issue', async () => {
      vi.spyOn(
        mockRpcClient,
        'getDictionaryItemByIdentifier'
      ).mockRejectedValue(new Error('Unexpected RPC error'));

      await expect(
        client.allowances(keyOwner.publicKey, keySpender.publicKey)
      ).rejects.toThrow('Unexpected RPC error');
    });

    it('should construct the correct dictionary key', async () => {
      await client.allowances(keyOwner.publicKey, keySpender.publicKey);

      const dictionaryIdentifier = (mockRpcClient as any)
        .getDictionaryItemByIdentifier.mock.calls[0][1];

      expect(dictionaryIdentifier.contractNamedKey).toEqual({
        key: contractHash,
        dictionaryName: 'allowances',
        dictionaryItemKey: expect.any(String)
      });
    });

    it('should throw an error when contract hash is not set in allowances', async () => {
      const clientWithoutContractHash = new CEP18Client('http://mock-rpc-url');
      await expect(
        clientWithoutContractHash.allowances(
          keyOwner.publicKey,
          keySpender.publicKey
        )
      ).rejects.toThrow('Contract hash is not set.');
    });
  });

  describe('CEP18Client - Getter Methods', () => {
    let client: CEP18Client;
    const mockName = 'CEP18';
    const mockSymbol = 'CEP18';
    const mockDecimals = 9;
    const mockTotalSupply = String(1000000);
    const mockEventsMode = EVENTS_MODE.NoEvents;
    const mockIsMintAndBurnEnabled = true;

    beforeEach(() => {
      client = new CEP18Client('http://mock-rpc-url');
    });

    it('should return the correct name', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue(mockName);
      const name = await client.name();
      expect(name).toBe(mockName);
    });

    it('should return the correct symbol', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue(
        mockSymbol
      );
      const symbol = await client.symbol();
      expect(symbol).toBe(mockSymbol);
    });

    it('should return the correct decimals', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue(
        mockDecimals
      );
      const decimals = await client.decimals();
      expect(decimals).toBe(mockDecimals);
    });

    it('should return the correct total supply', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue(
        mockTotalSupply
      );
      const totalSupply = await client.totalSupply();
      expect(totalSupply).toBe(mockTotalSupply);
    });

    it('should return the correct events mode', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue(
        mockEventsMode
      );
      const eventsMode = await client.eventsMode();
      expect(eventsMode).toBe('NoEvents');
    });

    it('should return true when mint and burn are enabled', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue(
        mockIsMintAndBurnEnabled
      );
      const isMintAndBurnEnabled = await client.isMintAndBurnEnabled();
      expect(isMintAndBurnEnabled).toBe(true);
    });

    it('should return false when mint and burn are not enabled', async () => {
      vi.spyOn(client as any, 'queryContractData').mockResolvedValue('0');
      const isMintAndBurnEnabled = await client.isMintAndBurnEnabled();
      expect(isMintAndBurnEnabled).toBe(false);
    });

    it('should throw an error if queryContractData fails', async () => {
      vi.spyOn(client as any, 'queryContractData').mockRejectedValue(
        new Error('Query failed')
      );
      await expect(client.name()).rejects.toThrow('Query failed');
    });
  });
});

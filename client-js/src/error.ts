export enum ERROR_CODES {
  /// CEP-18 contract called from within an invalid context.
  InvalidContext = 60000,
  /// Spender does not have enough balance.
  InsufficientBalance = 60001,
  /// Spender does not have enough allowance approved.
  InsufficientAllowance = 60002,
  /// Operation would cause an integer overflow.
  Overflow = 60003,
  /// A required package hash was not specified.
  PackageHashMissing = 60004,
  /// The package hash specified does not represent a package.
  PackageHashNotPackage = 60005,
  /// An invalid event mode was specified.
  InvalidEventsMode = 60006,
  /// The event mode required was not specified.
  MissingEventsMode = 60007,
  /// An unknown error occurred.
  Phantom = 60008,
  /// Failed to read the runtime arguments provided.
  FailedToGetArgBytes = 60009,
  /// The caller does not have sufficient security access.
  InsufficientRights = 60010,
  /// The list of Admin accounts provided is invalid.
  InvalidAdminList = 60011,
  /// The list of accounts that can mint tokens is invalid.
  InvalidMinterList = 60012,
  /// The list of accounts with no access rights is invalid.
  InvalidNoneList = 60013,
  /// The flag to enable the mint and burn mode is invalid.
  InvalidEnableMBFlag = 60014,
  /// This contract instance cannot be initialized again.
  AlreadyInitialized = 60015,
  /// The mint and burn mode is disabled.
  MintBurnDisabled = 60016,
  /// The target user cannot be the sender.
  CannotTargetSelfUser = 60017,
  /// The burn operation was attempted on an invalid target.
  InvalidBurnTarget = 60018,
  /// A required package hash for contract upgrade is missing.
  MissingPackageHashForUpgrade = 60019,
  /// A required contract hash for contract upgrade is missing.
  MissingContractHashForUpgrade = 60020,
  /// The provided key type is invalid.
  InvalidKeyType = 60021,
  /// Failed to convert data to JSON format.
  FailedToConvertToJson = 60022,
  /// Failed to return the expected entry point result.
  FailedToReturnEntryPointResult = 60023,
  /// Failed to create a new dictionary in storage.
  FailedToCreateDictionary = 60024,
  /// Failed to convert bytes to the expected type.
  FailedToConvertBytes = 60025,
  /// Failed to update the total supply of tokens.
  FailedToChangeTotalSupply = 60026,
  /// Failed to read data from storage.
  FailedToReadFromStorage = 60027,
  /// Failed to retrieve a key from storage.
  FailedToGetKey = 60028,
  /// Failed to disable a specific contract version.
  FailedToDisableContractVersion = 60029,
  /// Failed to insert an entry into the security list.
  FailedToInsertToSecurityList = 60030,
  /// The specified URef was not found.
  UrefNotFound = 60031,
  /// Failed to retrieve the old contract hash key.
  FailedToGetOldContractHashKey = 60032,
  /// Failed to retrieve the old package key.
  FailedToGetOldPackageKey = 60033,
  /// Failed to retrieve the package key.
  FailedToGetPackageKey = 60034,
  /// A required storage URef is missing.
  MissingStorageUref = 60035,
  /// The provided storage URef is invalid.
  InvalidStorageUref = 60036,
  /// Unable to retrieve the version contract hash key.
  MissingVersionContractKey = 60037,
  /// The provided version contract key is invalid.
  InvalidVersionContractKey = 60038
}

export class ContractError extends Error {
  constructor(public code: number) {
    super(ERROR_CODES[code]);
  }
}

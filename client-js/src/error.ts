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
  /// User cannot target themselves with allowance features.
  CannotTargetSelfUser = 60017,
  /// Tried to burn the tokens of someone else.
  InvalidBurnTarget = 60018,
  /// There is an AccessToken but no valid ContractPackageHash. Try to query the account for NamedKeys to find it.
  MissingPackageHashForUpgrade = 60019,
}

export class ContractError extends Error {
  constructor(public code: number) {
    super(ERROR_CODES[code]);
  }
}

export enum ERROR_CODES {
  /// CEP18 contract called from within an invalid context.
  InvalidContext = 60000,
  /// Spender does not have enough balance.
  InsufficientBalance = 60001,
  /// Spender does not have enough allowance approved.
  InsufficientAllowance = 60002,
  /// Operation would cause an integer overflow.
  Overflow = 60003,
  PackageHashMissing = 60004,
  PackageHashNotPackage = 60005,
  InvalidEventsMode = 60006,
  MissingEventsMode = 60007,
  Phantom = 60008,
  FailedToGetArgBytes = 60009,
  InsufficientRights = 60010,
  InvalidAdminList = 60011,
  InvalidMinterList = 60012,
  InvalidBurnerList = 60013,
  InvalidMintAndBurnList = 60014,
  InvalidNoneList = 60015,
  InvalidEnableMBFlag = 60016,
  AlreadyInitialized = 60017,
  MintBurnDisabled = 60018
}

export class ContractError extends Error {
  constructor(public code: number) {
    super(ERROR_CODES[code]);
  }
}

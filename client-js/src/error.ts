const MAX_U16 = 65535;

export enum ERROR_CODES {
  ERROR_INVALID_CONTEXT = MAX_U16,
  ERROR_INSUFFICIENT_BALANCE = MAX_U16 - 1,
  ERROR_INSUFFICIENT_ALLOWANCE = MAX_U16 - 2,
  ERROR_OVERFLOW = MAX_U16 - 3
}

export class ContractError extends Error {
  constructor(public code: number) {
    super(ERROR_CODES[code]);
  }
}

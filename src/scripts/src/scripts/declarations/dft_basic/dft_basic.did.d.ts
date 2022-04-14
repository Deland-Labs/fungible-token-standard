import type { Principal } from "@dfinity/principal";
export interface ArchiveInfo {
  startBlockHeight: bigint;
  numBlocks: bigint;
  canisterId: Principal;
  endBlockHeight: bigint;
}
export interface ArchivedBlocksRange {
  storageCanisterId: Principal;
  start: bigint;
  length: bigint;
}
export interface Block {
  transaction: Transaction;
  timestamp: bigint;
  parentHash: [] | [Array<number>];
}
export type BlockResult =
  | { Ok: Block }
  | { Err: ErrorInfo }
  | { Forward: Principal };
export type BooleanResult = { Ok: boolean } | { Err: ErrorInfo };
export interface ErrorInfo {
  code: number;
  message: string;
}
export type Operation =
  | {
      FeeToModify: { newFeeTo: TokenHolder; caller: Principal };
    }
  | {
      Approve: {
        fee: bigint;
        value: bigint;
        owner: TokenHolder;
        caller: Principal;
        spender: TokenHolder;
      };
    }
  | { FeeModify: { newFee: TokenFee; caller: Principal } }
  | {
      Transfer: {
        to: TokenHolder;
        fee: bigint;
        value: bigint;
        from: TokenHolder;
        caller: TokenHolder;
      };
    }
  | { OwnerModify: { newOwner: Principal; caller: Principal } };
export type OperationResult =
  | {
      Ok: {
        txId: string;
        error: [] | [ErrorInfo];
        blockHeight: bigint;
      };
    }
  | { Err: ErrorInfo };
export interface QueryBlocksResult {
  chainLength: bigint;
  certificate: [] | [Array<number>];
  archivedBlocks: Array<ArchivedBlocksRange>;
  blocks: Array<Block>;
  firstBlockIndex: bigint;
}
export interface TokenFee {
  rate: bigint;
  minimum: bigint;
  rateDecimals: number;
}
export type TokenHolder =
  | { None: null }
  | { Account: string }
  | { Principal: Principal };
export interface TokenInfo {
  certificate: [] | [Array<number>];
  owner: Principal;
  allowanceSize: bigint;
  cycles: bigint;
  blockHeight: bigint;
  holders: bigint;
  storages: Array<Principal>;
  feeTo: TokenHolder;
}
export interface TokenMetadata {
  fee: TokenFee;
  decimals: number;
  name: string;
  symbol: string;
}
export interface Transaction {
  createdAt: bigint;
  operation: Operation;
}
export interface _SERVICE {
  allowance: (arg_0: string, arg_1: string) => Promise<bigint>;
  allowancesOf: (arg_0: string) => Promise<Array<[TokenHolder, bigint]>>;
  approve: (
    arg_0: [] | [Array<number>],
    arg_1: string,
    arg_2: bigint,
    arg_3: [] | [bigint]
  ) => Promise<OperationResult>;
  archives: () => Promise<Array<ArchiveInfo>>;
  balanceOf: (arg_0: string) => Promise<bigint>;
  blockByHeight: (arg_0: bigint) => Promise<BlockResult>;
  blocksByQuery: (arg_0: bigint, arg_1: bigint) => Promise<QueryBlocksResult>;
  decimals: () => Promise<number>;
  desc: () => Promise<Array<[string, string]>>;
  fee: () => Promise<TokenFee>;
  logo: () => Promise<Array<number>>;
  meta: () => Promise<TokenMetadata>;
  name: () => Promise<string>;
  owner: () => Promise<Principal>;
  setDesc: (arg_0: Array<[string, string]>) => Promise<BooleanResult>;
  setFee: (arg_0: TokenFee, arg_1: [] | [bigint]) => Promise<BooleanResult>;
  setFeeTo: (arg_0: string, arg_1: [] | [bigint]) => Promise<BooleanResult>;
  setLogo: (arg_0: [] | [Array<number>]) => Promise<BooleanResult>;
  setOwner: (arg_0: Principal, arg_1: [] | [bigint]) => Promise<BooleanResult>;
  symbol: () => Promise<string>;
  tokenInfo: () => Promise<TokenInfo>;
  totalSupply: () => Promise<bigint>;
  transfer: (
    arg_0: [] | [Array<number>],
    arg_1: string,
    arg_2: bigint,
    arg_3: [] | [bigint]
  ) => Promise<OperationResult>;
  transferFrom: (
    arg_0: [] | [Array<number>],
    arg_1: string,
    arg_2: string,
    arg_3: bigint,
    arg_4: [] | [bigint]
  ) => Promise<OperationResult>;
}

import type { Principal } from '@dfinity/principal';
export interface ArchiveInfo {
  'startBlockHeight' : bigint,
  'numBlocks' : bigint,
  'canisterId' : Principal,
  'endBlockHeight' : bigint,
}
export interface ArchiveOptions {
  'num_blocks_to_archive' : number,
  'trigger_threshold' : number,
  'max_message_size_bytes' : [] | [number],
  'cycles_for_archive_creation' : [] | [bigint],
  'node_max_memory_size_bytes' : [] | [number],
}
export interface ArchivedBlocksRange {
  'storageCanisterId' : Principal,
  'start' : bigint,
  'length' : bigint,
}
export type BlockResult = { 'Ok' : CandidBlock } |
  { 'Err' : ErrorInfo } |
  { 'Forward' : Principal };
export type BooleanResult = { 'Ok' : boolean } |
  { 'Err' : ErrorInfo };
export interface CandidBlock {
  'transaction' : CandidTransaction,
  'timestamp' : bigint,
  'parentHash' : Array<number>,
}
export type CandidOperation = {
    'FeeToModify' : { 'newFeeTo' : string, 'caller' : Principal }
  } |
  {
    'Approve' : {
      'fee' : bigint,
      'value' : bigint,
      'owner' : string,
      'caller' : Principal,
      'spender' : string,
    }
  } |
  { 'RemoveMinter' : { 'minter' : Principal, 'caller' : Principal } } |
  { 'FeeModify' : { 'newFee' : CandidTokenFee, 'caller' : Principal } } |
  { 'AddMinter' : { 'minter' : Principal, 'caller' : Principal } } |
  {
    'Transfer' : {
      'to' : string,
      'fee' : bigint,
      'value' : bigint,
      'from' : string,
      'caller' : string,
    }
  } |
  { 'OwnerModify' : { 'newOwner' : Principal, 'caller' : Principal } };
export interface CandidTokenFee {
  'rate' : number,
  'minimum' : bigint,
  'rateDecimals' : number,
}
export interface CandidTokenMetadata {
  'fee' : CandidTokenFee,
  'decimals' : number,
  'name' : string,
  'symbol' : string,
}
export interface CandidTransaction {
  'createdAt' : bigint,
  'operation' : CandidOperation,
}
export interface ErrorInfo { 'code' : number, 'message' : string }
export interface HttpRequest {
  'url' : string,
  'method' : string,
  'body' : Array<number>,
  'headers' : Array<[string, string]>,
}
export interface HttpResponse {
  'body' : Array<number>,
  'headers' : Array<[string, string]>,
  'streaming_strategy' : [] | [StreamingStrategy],
  'status_code' : number,
}
export type OperationResult = {
    'Ok' : { 'txId' : string, 'blockHeight' : bigint }
  } |
  { 'Err' : ErrorInfo };
export interface QueryBlocksResult {
  'chainLength' : bigint,
  'certificate' : [] | [Array<number>],
  'archivedBlocks' : Array<ArchivedBlocksRange>,
  'blocks' : Array<CandidBlock>,
  'firstBlockIndex' : bigint,
}
export type StreamingStrategy = {
    'Callback' : { 'token' : {}, 'callback' : [Principal, string] }
  };
export interface TokenInfo {
  'fee' : CandidTokenFee,
  'certificate' : [] | [Array<number>],
  'owner' : Principal,
  'allowanceSize' : bigint,
  'blockHeight' : bigint,
  'holders' : bigint,
  'archiveCanisters' : Array<Principal>,
  'feeTo' : string,
}
export interface TokenMetrics {
  'chainLength' : bigint,
  'certificate' : [] | [Array<number>],
  'allowanceSize' : bigint,
  'localBlockCount' : bigint,
  'holders' : bigint,
  'cyclesBalance' : bigint,
}
export interface _SERVICE {
  'allowance' : (arg_0: string, arg_1: string) => Promise<bigint>,
  'allowancesOf' : (arg_0: string) => Promise<Array<[string, bigint]>>,
  'approve' : (
      arg_0: [] | [Array<number>],
      arg_1: string,
      arg_2: bigint,
      arg_3: [] | [bigint],
    ) => Promise<OperationResult>,
  'archives' : () => Promise<Array<ArchiveInfo>>,
  'balanceOf' : (arg_0: string) => Promise<bigint>,
  'blockByHeight' : (arg_0: bigint) => Promise<BlockResult>,
  'blocksByQuery' : (arg_0: bigint, arg_1: bigint) => Promise<
      QueryBlocksResult
    >,
  'decimals' : () => Promise<number>,
  'desc' : () => Promise<Array<[string, string]>>,
  'fee' : () => Promise<CandidTokenFee>,
  'http_request' : (arg_0: HttpRequest) => Promise<HttpResponse>,
  'logo' : () => Promise<Array<number>>,
  'meta' : () => Promise<CandidTokenMetadata>,
  'name' : () => Promise<string>,
  'owner' : () => Promise<Principal>,
  'setDesc' : (arg_0: Array<[string, string]>) => Promise<BooleanResult>,
  'setFee' : (arg_0: CandidTokenFee, arg_1: [] | [bigint]) => Promise<
      BooleanResult
    >,
  'setFeeTo' : (arg_0: string, arg_1: [] | [bigint]) => Promise<BooleanResult>,
  'setLogo' : (arg_0: [] | [Array<number>]) => Promise<BooleanResult>,
  'setOwner' : (arg_0: Principal, arg_1: [] | [bigint]) => Promise<
      BooleanResult
    >,
  'symbol' : () => Promise<string>,
  'tokenInfo' : () => Promise<TokenInfo>,
  'tokenMetrics' : () => Promise<TokenMetrics>,
  'totalSupply' : () => Promise<bigint>,
  'transfer' : (
      arg_0: [] | [Array<number>],
      arg_1: string,
      arg_2: bigint,
      arg_3: [] | [bigint],
    ) => Promise<OperationResult>,
  'transferFrom' : (
      arg_0: [] | [Array<number>],
      arg_1: string,
      arg_2: string,
      arg_3: bigint,
      arg_4: [] | [bigint],
    ) => Promise<OperationResult>,
}

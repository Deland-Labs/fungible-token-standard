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
export interface Block {
  'transaction' : Transaction,
  'timestamp' : bigint,
  'parentHash' : Array<number>,
}
export type BlockResult = { 'Ok' : Block } |
  { 'Err' : ErrorInfo } |
  { 'Forward' : Principal };
export type BooleanResult = { 'Ok' : boolean } |
  { 'Err' : ErrorInfo };
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
export type Operation = {
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
  { 'FeeModify' : { 'newFee' : TokenFee, 'caller' : Principal } } |
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
export type OperationResult = {
    'Ok' : { 'txId' : string, 'blockHeight' : bigint }
  } |
  { 'Err' : ErrorInfo };
export interface QueryBlocksResult {
  'chainLength' : bigint,
  'certificate' : [] | [Array<number>],
  'archivedBlocks' : Array<ArchivedBlocksRange>,
  'blocks' : Array<Block>,
  'firstBlockIndex' : bigint,
}
export type StreamingStrategy = {
    'Callback' : { 'token' : {}, 'callback' : [Principal, string] }
  };
export interface TokenFee {
  'rate' : number,
  'minimum' : bigint,
  'rateDecimals' : number,
}
export interface TokenInfo {
  'fee' : TokenFee,
  'chainLength' : bigint,
  'certificate' : [] | [Array<number>],
  'owner' : Principal,
  'allowanceSize' : bigint,
  'holders' : bigint,
  'archiveCanisters' : Array<Principal>,
  'feeTo' : string,
}
export interface TokenMetadata {
  'fee' : TokenFee,
  'decimals' : number,
  'name' : string,
  'symbol' : string,
}
export interface TokenMetrics {
  'chainLength' : bigint,
  'certificate' : [] | [Array<number>],
  'allowanceSize' : bigint,
  'localBlockCount' : bigint,
  'holders' : bigint,
  'cyclesBalance' : bigint,
}
export interface Transaction { 'createdAt' : bigint, 'operation' : Operation }
export interface _SERVICE {
  'addMinter' : (arg_0: Principal, arg_1: [] | [bigint]) => Promise<
      BooleanResult
    >,
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
  'fee' : () => Promise<TokenFee>,
  'http_request' : (arg_0: HttpRequest) => Promise<HttpResponse>,
  'logo' : () => Promise<Array<number>>,
  'meta' : () => Promise<TokenMetadata>,
  'mint' : (arg_0: string, arg_1: bigint, arg_2: [] | [bigint]) => Promise<
      OperationResult
    >,
  'minters' : () => Promise<Array<Principal>>,
  'name' : () => Promise<string>,
  'owner' : () => Promise<Principal>,
  'removeMinter' : (arg_0: Principal, arg_1: [] | [bigint]) => Promise<
      BooleanResult
    >,
  'setDesc' : (arg_0: Array<[string, string]>) => Promise<BooleanResult>,
  'setFee' : (arg_0: TokenFee, arg_1: [] | [bigint]) => Promise<BooleanResult>,
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

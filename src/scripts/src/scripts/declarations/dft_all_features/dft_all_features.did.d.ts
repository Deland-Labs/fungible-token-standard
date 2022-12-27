import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

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
  'parentHash' : Uint8Array,
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
  'body' : Uint8Array,
  'headers' : Array<[string, string]>,
}
export interface HttpResponse {
  'body' : Uint8Array,
  'headers' : Array<[string, string]>,
  'streaming_strategy' : [] | [StreamingStrategy],
  'status_code' : number,
}
export type Operation = {
    'FeeToModify' : { 'newFeeTo' : string, 'caller' : string }
  } |
  {
    'Approve' : {
      'fee' : bigint,
      'value' : bigint,
      'owner' : string,
      'caller' : string,
      'spender' : string,
    }
  } |
  { 'RemoveMinter' : { 'minter' : string, 'caller' : string } } |
  { 'FeeModify' : { 'newFee' : TokenFee, 'caller' : string } } |
  { 'AddMinter' : { 'minter' : string, 'caller' : string } } |
  {
    'Transfer' : {
      'to' : string,
      'fee' : bigint,
      'value' : bigint,
      'from' : string,
      'caller' : string,
    }
  } |
  { 'OwnerModify' : { 'newOwner' : string, 'caller' : string } };
export type OperationResult = {
    'Ok' : { 'txId' : string, 'blockHeight' : bigint }
  } |
  { 'Err' : ErrorInfo };
export interface QueryBlocksResult {
  'chainLength' : bigint,
  'certificate' : [] | [Uint8Array],
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
  'certificate' : [] | [Uint8Array],
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
  'certificate' : [] | [Uint8Array],
  'allowanceSize' : bigint,
  'localBlockCount' : bigint,
  'holders' : bigint,
  'cyclesBalance' : bigint,
}
export interface Transaction { 'createdAt' : bigint, 'operation' : Operation }
export interface _SERVICE {
  'addMinter' : ActorMethod<[Principal, [] | [bigint]], BooleanResult>,
  'allowance' : ActorMethod<[string, string], bigint>,
  'allowancesOf' : ActorMethod<[string], Array<[string, bigint]>>,
  'approve' : ActorMethod<
    [[] | [Uint8Array], string, bigint, [] | [bigint]],
    OperationResult
  >,
  'archives' : ActorMethod<[], Array<ArchiveInfo>>,
  'balanceOf' : ActorMethod<[string], bigint>,
  'batchMint' : ActorMethod<
    [Array<[string, bigint]>, [] | [bigint]],
    Array<OperationResult>
  >,
  'batchTransfer' : ActorMethod<
    [[] | [Uint8Array], Array<[string, bigint]>, [] | [bigint]],
    Array<OperationResult>
  >,
  'batchTransferFrom' : ActorMethod<
    [[] | [Uint8Array], string, Array<[string, bigint]>, [] | [bigint]],
    Array<OperationResult>
  >,
  'blockByHeight' : ActorMethod<[bigint], BlockResult>,
  'blocksByQuery' : ActorMethod<[bigint, bigint], QueryBlocksResult>,
  'burn' : ActorMethod<
    [[] | [Uint8Array], bigint, [] | [bigint]],
    OperationResult
  >,
  'burnFrom' : ActorMethod<
    [[] | [Uint8Array], string, bigint, [] | [bigint]],
    OperationResult
  >,
  'decimals' : ActorMethod<[], number>,
  'desc' : ActorMethod<[], Array<[string, string]>>,
  'fee' : ActorMethod<[], TokenFee>,
  'http_request' : ActorMethod<[HttpRequest], HttpResponse>,
  'logo' : ActorMethod<[], Uint8Array>,
  'meta' : ActorMethod<[], TokenMetadata>,
  'mint' : ActorMethod<[string, bigint, [] | [bigint]], OperationResult>,
  'minters' : ActorMethod<[], Array<Principal>>,
  'name' : ActorMethod<[], string>,
  'owner' : ActorMethod<[], Principal>,
  'removeMinter' : ActorMethod<[Principal, [] | [bigint]], BooleanResult>,
  'setDesc' : ActorMethod<[Array<[string, string]>], BooleanResult>,
  'setFee' : ActorMethod<[TokenFee, [] | [bigint]], BooleanResult>,
  'setFeeTo' : ActorMethod<[string, [] | [bigint]], BooleanResult>,
  'setLogo' : ActorMethod<[[] | [Uint8Array]], BooleanResult>,
  'setOwner' : ActorMethod<[Principal, [] | [bigint]], BooleanResult>,
  'symbol' : ActorMethod<[], string>,
  'tokenInfo' : ActorMethod<[], TokenInfo>,
  'tokenMetrics' : ActorMethod<[], TokenMetrics>,
  'totalSupply' : ActorMethod<[], bigint>,
  'transfer' : ActorMethod<
    [[] | [Uint8Array], string, bigint, [] | [bigint]],
    OperationResult
  >,
  'transferFrom' : ActorMethod<
    [[] | [Uint8Array], string, string, bigint, [] | [bigint]],
    OperationResult
  >,
}

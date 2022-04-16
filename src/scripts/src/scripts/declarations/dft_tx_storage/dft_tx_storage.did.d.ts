import type { Principal } from '@dfinity/principal';
export type BlockListResult = { 'Ok' : Array<CandidBlock> } |
  { 'Err' : ErrorInfo };
export type BlockResult = { 'Ok' : CandidBlock } |
  { 'Err' : ErrorInfo } |
  { 'Forward' : Principal };
export type BooleanResult = { 'Ok' : boolean } |
  { 'Err' : ErrorInfo };
export interface CandidBlock {
  'transaction' : CandidTransaction,
  'timestamp' : bigint,
  'parentHash' : [] | [Array<number>],
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
  { 'FeeModify' : { 'newFee' : CandidTokenFee, 'caller' : Principal } } |
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
export interface CandidTransaction {
  'createdAt' : bigint,
  'operation' : CandidOperation,
}
export interface ErrorInfo { 'code' : number, 'message' : string }
export interface StorageInfo {
  'tokenId' : Principal,
  'totalBlocksCount' : bigint,
  'cycles' : bigint,
  'totalBlockSizeBytes' : bigint,
  'blockHeightOffset' : bigint,
}
export interface _SERVICE {
  'batchAppend' : (arg_0: Array<Array<number>>) => Promise<BooleanResult>,
  'blockByHeight' : (arg_0: bigint) => Promise<BlockResult>,
  'blocksByQuery' : (arg_0: bigint, arg_1: bigint) => Promise<BlockListResult>,
  'storageInfo' : () => Promise<StorageInfo>,
}

import type { Principal } from '@dfinity/principal';
export interface ActorError { 'code' : number, 'message' : string }
export interface Fee {
  'rate' : bigint,
  'minimum' : bigint,
  'rate_decimals' : number,
}
export type Result = { 'Ok' : boolean } |
  { 'Err' : ActorError };
export type Result_1 = { 'Ok' : TxRecord } |
  { 'Err' : ActorError };
export type Result_2 = { 'Ok' : Array<TxRecord> } |
  { 'Err' : ActorError };
export interface StorageInfo {
  'dft_id' : Principal,
  'tx_start_index' : bigint,
  'txs_count' : bigint,
  'cycles' : bigint,
}
export type TokenHolder = { 'None' : null } |
  { 'Account' : string } |
  { 'Principal' : Principal };
export type TxRecord = {
    'FeeToModify' : [bigint, Principal, TokenHolder, bigint, bigint]
  } |
  {
    'Approve' : [
      bigint,
      TokenHolder,
      TokenHolder,
      TokenHolder,
      bigint,
      bigint,
      bigint,
      bigint,
    ]
  } |
  { 'FeeModify' : [bigint, Principal, Fee, bigint, bigint] } |
  {
    'Transfer' : [
      bigint,
      TokenHolder,
      TokenHolder,
      TokenHolder,
      bigint,
      bigint,
      bigint,
      bigint,
    ]
  } |
  { 'OwnerModify' : [bigint, Principal, Principal, bigint, bigint] };
export interface _SERVICE {
  'append' : (arg_0: TxRecord) => Promise<Result>,
  'batchAppend' : (arg_0: Array<TxRecord>) => Promise<Result>,
  'storageInfo' : () => Promise<StorageInfo>,
  'transactionById' : (arg_0: string) => Promise<Result_1>,
  'transactionByIndex' : (arg_0: bigint) => Promise<Result_1>,
  'transactions' : (arg_0: bigint, arg_1: bigint) => Promise<Result_2>,
}

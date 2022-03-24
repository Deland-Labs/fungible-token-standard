import type { Principal } from '@dfinity/principal';
export interface ActorError { 'code' : number, 'message' : string }
export type BooleanResult = { 'Ok' : boolean } |
  { 'Err' : ActorError };
export interface Fee {
  'rate' : bigint,
  'minimum' : bigint,
  'rate_decimals' : number,
}
export interface Metadata {
  'fee' : Fee,
  'decimals' : number,
  'name' : string,
  'totalSupply' : bigint,
  'symbol' : string,
}
export type TokenHolder = { 'None' : null } |
  { 'Account' : string } |
  { 'Principal' : Principal };
export interface TokenInfo {
  'owner' : Principal,
  'allowanceSize' : bigint,
  'cycles' : bigint,
  'txCount' : bigint,
  'holders' : bigint,
  'storages' : Array<Principal>,
  'feeTo' : TokenHolder,
}
export interface TransactionResponse {
  'txId' : string,
  'error' : [] | [ActorError],
}
export type TransactionResult = { 'Ok' : TransactionResponse } |
  { 'Err' : ActorError };
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
export type TxRecordListResult = { 'Ok' : Array<TxRecord> } |
  { 'Err' : ActorError };
export type TxRecordResult = { 'Ok' : TxRecord } |
  { 'Err' : ActorError } |
  { 'Forward' : Principal };
export interface _SERVICE {
  'allowance' : (arg_0: string, arg_1: string) => Promise<bigint>,
  'allowancesOf' : (arg_0: string) => Promise<Array<[TokenHolder, bigint]>>,
  'approve' : (
      arg_0: [] | [Array<number>],
      arg_1: string,
      arg_2: bigint,
      arg_3: [] | [bigint],
    ) => Promise<TransactionResult>,
  'balanceOf' : (arg_0: string) => Promise<bigint>,
  'burn' : (
      arg_0: [] | [Array<number>],
      arg_1: bigint,
      arg_2: [] | [bigint],
    ) => Promise<TransactionResult>,
  'burnFrom' : (
      arg_0: [] | [Array<number>],
      arg_1: string,
      arg_2: bigint,
      arg_3: [] | [bigint],
    ) => Promise<TransactionResult>,
  'decimals' : () => Promise<number>,
  'desc' : () => Promise<Array<[string, string]>>,
  'fee' : () => Promise<Fee>,
  'lastTransactions' : (arg_0: bigint) => Promise<TxRecordListResult>,
  'logo' : () => Promise<Array<number>>,
  'meta' : () => Promise<Metadata>,
  'name' : () => Promise<string>,
  'nonceOf' : (arg_0: Principal) => Promise<bigint>,
  'owner' : () => Promise<Principal>,
  'setDesc' : (arg_0: Array<[string, string]>) => Promise<BooleanResult>,
  'setFee' : (arg_0: Fee, arg_1: [] | [bigint]) => Promise<BooleanResult>,
  'setFeeTo' : (arg_0: string, arg_1: [] | [bigint]) => Promise<BooleanResult>,
  'setLogo' : (arg_0: [] | [Array<number>]) => Promise<BooleanResult>,
  'setOwner' : (arg_0: Principal, arg_1: [] | [bigint]) => Promise<
      BooleanResult
    >,
  'symbol' : () => Promise<string>,
  'tokenInfo' : () => Promise<TokenInfo>,
  'totalSupply' : () => Promise<bigint>,
  'transactionById' : (arg_0: string) => Promise<TxRecordResult>,
  'transactionByIndex' : (arg_0: bigint) => Promise<TxRecordResult>,
  'transfer' : (
      arg_0: [] | [Array<number>],
      arg_1: string,
      arg_2: bigint,
      arg_3: [] | [bigint],
    ) => Promise<TransactionResult>,
  'transferFrom' : (
      arg_0: [] | [Array<number>],
      arg_1: string,
      arg_2: string,
      arg_3: bigint,
      arg_4: [] | [bigint],
    ) => Promise<TransactionResult>,
}

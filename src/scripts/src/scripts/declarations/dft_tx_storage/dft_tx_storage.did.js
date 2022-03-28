export const idlFactory = ({ IDL }) => {
  const TokenHolder = IDL.Variant({
    'None' : IDL.Null,
    'Account' : IDL.Text,
    'Principal' : IDL.Principal,
  });
  const Fee = IDL.Record({
    'rate' : IDL.Nat,
    'minimum' : IDL.Nat,
    'rate_decimals' : IDL.Nat8,
  });
  const TxRecord = IDL.Variant({
    'FeeToModify' : IDL.Tuple(
      IDL.Nat,
      IDL.Principal,
      TokenHolder,
      IDL.Nat64,
      IDL.Nat64,
    ),
    'Approve' : IDL.Tuple(
      IDL.Nat,
      TokenHolder,
      TokenHolder,
      TokenHolder,
      IDL.Nat,
      IDL.Nat,
      IDL.Nat64,
      IDL.Nat64,
    ),
    'FeeModify' : IDL.Tuple(IDL.Nat, IDL.Principal, Fee, IDL.Nat64, IDL.Nat64),
    'Transfer' : IDL.Tuple(
      IDL.Nat,
      TokenHolder,
      TokenHolder,
      TokenHolder,
      IDL.Nat,
      IDL.Nat,
      IDL.Nat64,
      IDL.Nat64,
    ),
    'OwnerModify' : IDL.Tuple(
      IDL.Nat,
      IDL.Principal,
      IDL.Principal,
      IDL.Nat64,
      IDL.Nat64,
    ),
  });
  const ActorError = IDL.Record({ 'code' : IDL.Nat32, 'message' : IDL.Text });
  const BooleanResult = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : ActorError });
  const StorageInfo = IDL.Record({
    'dft_id' : IDL.Principal,
    'tx_start_index' : IDL.Nat,
    'txs_count' : IDL.Nat,
    'cycles' : IDL.Nat64,
  });
  const TxRecordResult = IDL.Variant({
    'Ok' : TxRecord,
    'Err' : ActorError,
    'Forward' : IDL.Principal,
  });
  const TxRecordListResult = IDL.Variant({
    'Ok' : IDL.Vec(TxRecord),
    'Err' : ActorError,
  });
  return IDL.Service({
    'append' : IDL.Func([TxRecord], [BooleanResult], []),
    'batchAppend' : IDL.Func([IDL.Vec(TxRecord)], [BooleanResult], []),
    'storageInfo' : IDL.Func([], [StorageInfo], ['query']),
    'transactionById' : IDL.Func([IDL.Text], [TxRecordResult], ['query']),
    'transactionByIndex' : IDL.Func([IDL.Nat], [TxRecordResult], ['query']),
    'transactions' : IDL.Func(
        [IDL.Nat, IDL.Nat64],
        [TxRecordListResult],
        ['query'],
      ),
  });
};
export const init = ({ IDL }) => { return []; };

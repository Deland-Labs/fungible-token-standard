export const idlFactory = ({ IDL }) => {
  const ErrorInfo = IDL.Record({ 'code' : IDL.Nat32, 'message' : IDL.Text });
  const BooleanResult = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : ErrorInfo });
  const TokenHolder = IDL.Variant({
    'None' : IDL.Null,
    'Account' : IDL.Text,
    'Principal' : IDL.Principal,
  });
  const TokenFee = IDL.Record({
    'rate' : IDL.Nat,
    'minimum' : IDL.Nat,
    'rateDecimals' : IDL.Nat8,
  });
  const Operation = IDL.Variant({
    'FeeToModify' : IDL.Record({
      'newFeeTo' : TokenHolder,
      'caller' : IDL.Principal,
    }),
    'Approve' : IDL.Record({
      'fee' : IDL.Nat,
      'value' : IDL.Nat,
      'owner' : TokenHolder,
      'caller' : IDL.Principal,
      'spender' : TokenHolder,
    }),
    'FeeModify' : IDL.Record({ 'newFee' : TokenFee, 'caller' : IDL.Principal }),
    'Transfer' : IDL.Record({
      'to' : TokenHolder,
      'fee' : IDL.Nat,
      'value' : IDL.Nat,
      'from' : TokenHolder,
      'caller' : TokenHolder,
    }),
    'OwnerModify' : IDL.Record({
      'newOwner' : IDL.Principal,
      'caller' : IDL.Principal,
    }),
  });
  const Transaction = IDL.Record({
    'createdAt' : IDL.Nat64,
    'operation' : Operation,
  });
  const Block = IDL.Record({
    'transaction' : Transaction,
    'timestamp' : IDL.Nat64,
    'parentHash' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const BlockResult = IDL.Variant({
    'Ok' : Block,
    'Err' : ErrorInfo,
    'Forward' : IDL.Principal,
  });
  const BlockListResult = IDL.Variant({
    'Ok' : IDL.Vec(Block),
    'Err' : ErrorInfo,
  });
  const StorageInfo = IDL.Record({
    'tokenId' : IDL.Principal,
    'totalBlocksCount' : IDL.Nat,
    'cycles' : IDL.Nat64,
    'totalBlockSizeBytes' : IDL.Nat,
    'blockHeightOffset' : IDL.Nat,
  });
  return IDL.Service({
    'batchAppend' : IDL.Func([IDL.Vec(IDL.Vec(IDL.Nat8))], [BooleanResult], []),
    'blockByIndex' : IDL.Func([IDL.Nat], [BlockResult], ['query']),
    'blocksByQuery' : IDL.Func(
        [IDL.Nat, IDL.Nat64],
        [BlockListResult],
        ['query'],
      ),
    'storageInfo' : IDL.Func([], [StorageInfo], ['query']),
  });
};
export const init = ({ IDL }) => { return [IDL.Principal, IDL.Nat]; };

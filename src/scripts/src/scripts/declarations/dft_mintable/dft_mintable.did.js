export const idlFactory = ({ IDL }) => {
  const Fee = IDL.Record({
    'rate' : IDL.Nat,
    'minimum' : IDL.Nat,
    'rate_decimals' : IDL.Nat8,
  });
  const TokenHolder = IDL.Variant({
    'None' : IDL.Null,
    'Account' : IDL.Text,
    'Principal' : IDL.Principal,
  });
  const ActorError = IDL.Record({ 'code' : IDL.Nat32, 'message' : IDL.Text });
  const TransactionResponse = IDL.Record({
    'txId' : IDL.Text,
    'error' : IDL.Opt(ActorError),
  });
  const TransactionResult = IDL.Variant({
    'Ok' : TransactionResponse,
    'Err' : ActorError,
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
  const TxRecordListResult = IDL.Variant({
    'Ok' : IDL.Vec(TxRecord),
    'Err' : ActorError,
  });
  const Metadata = IDL.Record({
    'fee' : Fee,
    'decimals' : IDL.Nat8,
    'name' : IDL.Text,
    'totalSupply' : IDL.Nat,
    'symbol' : IDL.Text,
  });
  const BooleanResult = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : ActorError });
  const TokenInfo = IDL.Record({
    'owner' : IDL.Principal,
    'allowanceSize' : IDL.Nat,
    'cycles' : IDL.Nat64,
    'txCount' : IDL.Nat,
    'holders' : IDL.Nat,
    'storages' : IDL.Vec(IDL.Principal),
    'feeTo' : TokenHolder,
  });
  const TxRecordResult = IDL.Variant({
    'Ok' : TxRecord,
    'Err' : ActorError,
    'Forward' : IDL.Principal,
  });
  return IDL.Service({
    'allowance' : IDL.Func([IDL.Text, IDL.Text], [IDL.Nat], ['query']),
    'allowancesOf' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(IDL.Tuple(TokenHolder, IDL.Nat))],
        ['query'],
      ),
    'approve' : IDL.Func(
        [IDL.Opt(IDL.Vec(IDL.Nat8)), IDL.Text, IDL.Nat, IDL.Opt(IDL.Nat64)],
        [TransactionResult],
        [],
      ),
    'balanceOf' : IDL.Func([IDL.Text], [IDL.Nat], ['query']),
    'decimals' : IDL.Func([], [IDL.Nat8], ['query']),
    'desc' : IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))], ['query']),
    'fee' : IDL.Func([], [Fee], ['query']),
    'lastTransactions' : IDL.Func([IDL.Nat64], [TxRecordListResult], ['query']),
    'logo' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'meta' : IDL.Func([], [Metadata], ['query']),
    'mint' : IDL.Func(
        [IDL.Text, IDL.Nat, IDL.Opt(IDL.Nat64)],
        [TransactionResult],
        [],
      ),
    'name' : IDL.Func([], [IDL.Text], ['query']),
    'nonceOf' : IDL.Func([IDL.Principal], [IDL.Nat64], ['query']),
    'owner' : IDL.Func([], [IDL.Principal], ['query']),
    'setDesc' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)), IDL.Opt(IDL.Nat64)],
        [BooleanResult],
        [],
      ),
    'setFee' : IDL.Func([Fee, IDL.Opt(IDL.Nat64)], [BooleanResult], []),
    'setFeeTo' : IDL.Func([IDL.Text, IDL.Opt(IDL.Nat64)], [BooleanResult], []),
    'setLogo' : IDL.Func(
        [IDL.Opt(IDL.Vec(IDL.Nat8)), IDL.Opt(IDL.Nat64)],
        [BooleanResult],
        [],
      ),
    'setOwner' : IDL.Func(
        [IDL.Principal, IDL.Opt(IDL.Nat64)],
        [BooleanResult],
        [],
      ),
    'symbol' : IDL.Func([], [IDL.Text], ['query']),
    'tokenInfo' : IDL.Func([], [TokenInfo], ['query']),
    'totalSupply' : IDL.Func([], [IDL.Nat], ['query']),
    'transactionById' : IDL.Func([IDL.Text], [TxRecordResult], ['query']),
    'transactionByIndex' : IDL.Func([IDL.Nat], [TxRecordResult], ['query']),
    'transfer' : IDL.Func(
        [IDL.Opt(IDL.Vec(IDL.Nat8)), IDL.Text, IDL.Nat, IDL.Opt(IDL.Nat64)],
        [TransactionResult],
        [],
      ),
    'transferFrom' : IDL.Func(
        [
          IDL.Opt(IDL.Vec(IDL.Nat8)),
          IDL.Text,
          IDL.Text,
          IDL.Nat,
          IDL.Opt(IDL.Nat64),
        ],
        [TransactionResult],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const Fee = IDL.Record({
    'rate' : IDL.Nat,
    'minimum' : IDL.Nat,
    'rate_decimals' : IDL.Nat8,
  });
  return [
    IDL.Opt(IDL.Vec(IDL.Nat8)),
    IDL.Opt(IDL.Vec(IDL.Nat8)),
    IDL.Text,
    IDL.Text,
    IDL.Nat8,
    IDL.Nat,
    Fee,
    IDL.Opt(IDL.Principal),
  ];
};

export const idlFactory = ({ IDL }) => {
  const TokenFee = IDL.Record({
    'rate': IDL.Nat,
    'minimum': IDL.Nat,
    'rateDecimals': IDL.Nat8,
  });
  const TokenHolder = IDL.Variant({
    'None': IDL.Null,
    'Account': IDL.Text,
    'Principal': IDL.Principal,
  });
  const ErrorInfo = IDL.Record({ 'code': IDL.Nat32, 'message': IDL.Text });
  const OperationResult = IDL.Variant({
    'Ok' : IDL.Record({
      'txId' : IDL.Text,
      'error' : IDL.Opt(ErrorInfo),
      'blockHeight' : IDL.Nat,
    }),
    'Err' : ErrorInfo,
  });
  const ArchiveInfo = IDL.Record({
    'startBlockHeight' : IDL.Nat,
    'numBlocks' : IDL.Nat,
    'canisterId' : IDL.Principal,
    'endBlockHeight' : IDL.Nat,
  });
  const Operation = IDL.Variant({
    'FeeToModify': IDL.Record({
      'newFeeTo': TokenHolder,
      'caller': IDL.Principal,
    }),
    'Approve': IDL.Record({
      'fee': IDL.Nat,
      'value': IDL.Nat,
      'owner': TokenHolder,
      'caller': IDL.Principal,
      'spender': TokenHolder,
    }),
    'FeeModify': IDL.Record({ 'newFee': TokenFee, 'caller': IDL.Principal }),
    'Transfer': IDL.Record({
      'to': TokenHolder,
      'fee': IDL.Nat,
      'value': IDL.Nat,
      'from': TokenHolder,
      'caller': TokenHolder,
    }),
    'OwnerModify': IDL.Record({
      'newOwner': IDL.Principal,
      'caller': IDL.Principal,
    }),
  });
  const Transaction = IDL.Record({
    'createdAt': IDL.Nat64,
    'operation': Operation,
  });
  const Block = IDL.Record({
    'transaction': Transaction,
    'timestamp': IDL.Nat64,
    'parentHash': IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const BlockResult = IDL.Variant({
    'Ok': Block,
    'Err': ErrorInfo,
    'Forward': IDL.Principal,
  });
  const ArchivedBlocksRange = IDL.Record({
    'storageCanisterId': IDL.Principal,
    'start': IDL.Nat,
    'length': IDL.Nat64,
  });
  const QueryBlocksResult = IDL.Record({
    'chainLength': IDL.Nat,
    'certificate': IDL.Opt(IDL.Vec(IDL.Nat8)),
    'archivedBlocks': IDL.Vec(ArchivedBlocksRange),
    'blocks': IDL.Vec(Block),
    'firstBlockIndex': IDL.Nat,
  });
  const TokenMetadata = IDL.Record({
    'fee': TokenFee,
    'decimals': IDL.Nat8,
    'name': IDL.Text,
    'symbol': IDL.Text,
  });
  const BooleanResult = IDL.Variant({ 'Ok': IDL.Bool, 'Err': ErrorInfo });
  const TokenInfo = IDL.Record({
    'certificate': IDL.Opt(IDL.Vec(IDL.Nat8)),
    'owner': IDL.Principal,
    'allowanceSize': IDL.Nat,
    'cycles': IDL.Nat64,
    'blockHeight': IDL.Nat,
    'holders': IDL.Nat,
    'storages': IDL.Vec(IDL.Principal),
    'feeTo': TokenHolder,
  });
  return IDL.Service({
    'allowance': IDL.Func([IDL.Text, IDL.Text], [IDL.Nat], ['query']),
    'allowancesOf': IDL.Func(
      [IDL.Text],
      [IDL.Vec(IDL.Tuple(TokenHolder, IDL.Nat))],
      ['query'],
    ),
    'approve': IDL.Func(
      [IDL.Opt(IDL.Vec(IDL.Nat8)), IDL.Text, IDL.Nat, IDL.Opt(IDL.Nat64)],
      [OperationResult],
      [],
    ),
    'archives' : IDL.Func([], [IDL.Vec(ArchiveInfo)], ['query']),
    'balanceOf': IDL.Func([IDL.Text], [IDL.Nat], ['query']),
    'blockByHeight': IDL.Func([IDL.Nat], [BlockResult], ['query']),
    'blocksByQuery': IDL.Func(
      [IDL.Nat, IDL.Nat64],
      [QueryBlocksResult],
      ['query'],
    ),
    'burn': IDL.Func(
      [IDL.Opt(IDL.Vec(IDL.Nat8)), IDL.Nat, IDL.Opt(IDL.Nat64)],
      [OperationResult],
      [],
    ),
    'burnFrom': IDL.Func(
      [IDL.Opt(IDL.Vec(IDL.Nat8)), IDL.Text, IDL.Nat, IDL.Opt(IDL.Nat64)],
      [OperationResult],
      [],
    ),
    'decimals': IDL.Func([], [IDL.Nat8], ['query']),
    'desc': IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))], ['query']),
    'fee': IDL.Func([], [TokenFee], ['query']),
    'logo': IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'meta': IDL.Func([], [TokenMetadata], ['query']),
    'name': IDL.Func([], [IDL.Text], ['query']),
    'owner': IDL.Func([], [IDL.Principal], ['query']),
    'setDesc': IDL.Func(
      [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))],
      [BooleanResult],
      [],
    ),
    'setFee': IDL.Func([TokenFee, IDL.Opt(IDL.Nat64)], [BooleanResult], []),
    'setFeeTo': IDL.Func([IDL.Text, IDL.Opt(IDL.Nat64)], [BooleanResult], []),
    'setLogo': IDL.Func([IDL.Opt(IDL.Vec(IDL.Nat8))], [BooleanResult], []),
    'setOwner': IDL.Func(
      [IDL.Principal, IDL.Opt(IDL.Nat64)],
      [BooleanResult],
      [],
    ),
    'symbol': IDL.Func([], [IDL.Text], ['query']),
    'tokenInfo': IDL.Func([], [TokenInfo], ['query']),
    'totalSupply': IDL.Func([], [IDL.Nat], ['query']),
    'transfer': IDL.Func(
      [IDL.Opt(IDL.Vec(IDL.Nat8)), IDL.Text, IDL.Nat, IDL.Opt(IDL.Nat64)],
      [OperationResult],
      [],
    ),
    'transferFrom': IDL.Func(
      [
        IDL.Opt(IDL.Vec(IDL.Nat8)),
        IDL.Text,
        IDL.Text,
        IDL.Nat,
        IDL.Opt(IDL.Nat64),
      ],
      [OperationResult],
      [],
    ),
  });
};
export const init = ({ IDL }) => {
  const TokenFee = IDL.Record({
    'rate': IDL.Nat,
    'minimum': IDL.Nat,
    'rateDecimals': IDL.Nat8,
  });
  return [
    IDL.Opt(IDL.Vec(IDL.Nat8)),
    IDL.Opt(IDL.Vec(IDL.Nat8)),
    IDL.Text,
    IDL.Text,
    IDL.Nat8,
    IDL.Nat,
    TokenFee,
    IDL.Opt(IDL.Principal),
  ];
};

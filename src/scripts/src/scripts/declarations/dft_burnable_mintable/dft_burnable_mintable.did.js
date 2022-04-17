export const idlFactory = ({ IDL }) => {
  const CandidTokenFee = IDL.Record({
    'rate' : IDL.Nat32,
    'minimum' : IDL.Nat,
    'rateDecimals' : IDL.Nat8,
  });
  const ArchiveOptions = IDL.Record({
    'num_blocks_to_archive' : IDL.Nat32,
    'trigger_threshold' : IDL.Nat32,
    'max_message_size_bytes' : IDL.Opt(IDL.Nat32),
    'cycles_for_archive_creation' : IDL.Opt(IDL.Nat64),
    'node_max_memory_size_bytes' : IDL.Opt(IDL.Nat32),
  });
  const ErrorInfo = IDL.Record({ 'code' : IDL.Nat32, 'message' : IDL.Text });
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
  const CandidOperation = IDL.Variant({
    'FeeToModify' : IDL.Record({
      'newFeeTo' : IDL.Text,
      'caller' : IDL.Principal,
    }),
    'Approve' : IDL.Record({
      'fee' : IDL.Nat,
      'value' : IDL.Nat,
      'owner' : IDL.Text,
      'caller' : IDL.Principal,
      'spender' : IDL.Text,
    }),
    'FeeModify' : IDL.Record({
      'newFee' : CandidTokenFee,
      'caller' : IDL.Principal,
    }),
    'Transfer' : IDL.Record({
      'to' : IDL.Text,
      'fee' : IDL.Nat,
      'value' : IDL.Nat,
      'from' : IDL.Text,
      'caller' : IDL.Text,
    }),
    'OwnerModify' : IDL.Record({
      'newOwner' : IDL.Principal,
      'caller' : IDL.Principal,
    }),
  });
  const CandidTransaction = IDL.Record({
    'createdAt' : IDL.Nat64,
    'operation' : CandidOperation,
  });
  const CandidBlock = IDL.Record({
    'transaction' : CandidTransaction,
    'timestamp' : IDL.Nat64,
    'parentHash' : IDL.Opt(IDL.Vec(IDL.Nat8)),
  });
  const BlockResult = IDL.Variant({
    'Ok' : CandidBlock,
    'Err' : ErrorInfo,
    'Forward' : IDL.Principal,
  });
  const ArchivedBlocksRange = IDL.Record({
    'storageCanisterId' : IDL.Principal,
    'start' : IDL.Nat,
    'length' : IDL.Nat64,
  });
  const QueryBlocksResult = IDL.Record({
    'chainLength' : IDL.Nat,
    'certificate' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'archivedBlocks' : IDL.Vec(ArchivedBlocksRange),
    'blocks' : IDL.Vec(CandidBlock),
    'firstBlockIndex' : IDL.Nat,
  });
  const HttpRequest = IDL.Record({
    'url' : IDL.Text,
    'method' : IDL.Text,
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
  });
  const StreamingStrategy = IDL.Variant({
    'Callback' : IDL.Record({
      'token' : IDL.Record({}),
      'callback' : IDL.Func([], [], []),
    }),
  });
  const HttpResponse = IDL.Record({
    'body' : IDL.Vec(IDL.Nat8),
    'headers' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
    'streaming_strategy' : IDL.Opt(StreamingStrategy),
    'status_code' : IDL.Nat16,
  });
  const CandidTokenMetadata = IDL.Record({
    'fee' : CandidTokenFee,
    'decimals' : IDL.Nat8,
    'name' : IDL.Text,
    'symbol' : IDL.Text,
  });
  const BooleanResult = IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : ErrorInfo });
  const TokenInfo = IDL.Record({
    'certificate' : IDL.Opt(IDL.Vec(IDL.Nat8)),
    'owner' : IDL.Principal,
    'allowanceSize' : IDL.Nat64,
    'cycles' : IDL.Nat64,
    'blockHeight' : IDL.Nat,
    'holders' : IDL.Nat64,
    'storages' : IDL.Vec(IDL.Principal),
    'feeTo' : IDL.Text,
  });
  return IDL.Service({
    'allowance' : IDL.Func([IDL.Text, IDL.Text], [IDL.Nat], ['query']),
    'allowancesOf' : IDL.Func(
        [IDL.Text],
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Nat))],
        ['query'],
      ),
    'approve' : IDL.Func(
        [IDL.Opt(IDL.Vec(IDL.Nat8)), IDL.Text, IDL.Nat, IDL.Opt(IDL.Nat64)],
        [OperationResult],
        [],
      ),
    'archives' : IDL.Func([], [IDL.Vec(ArchiveInfo)], ['query']),
    'balanceOf' : IDL.Func([IDL.Text], [IDL.Nat], ['query']),
    'blockByHeight' : IDL.Func([IDL.Nat], [BlockResult], ['query']),
    'blocksByQuery' : IDL.Func(
        [IDL.Nat, IDL.Nat64],
        [QueryBlocksResult],
        ['query'],
      ),
    'burn' : IDL.Func(
        [IDL.Opt(IDL.Vec(IDL.Nat8)), IDL.Nat, IDL.Opt(IDL.Nat64)],
        [OperationResult],
        [],
      ),
    'burnFrom' : IDL.Func(
        [IDL.Opt(IDL.Vec(IDL.Nat8)), IDL.Text, IDL.Nat, IDL.Opt(IDL.Nat64)],
        [OperationResult],
        [],
      ),
    'decimals' : IDL.Func([], [IDL.Nat8], ['query']),
    'desc' : IDL.Func([], [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))], ['query']),
    'fee' : IDL.Func([], [CandidTokenFee], ['query']),
    'http_request' : IDL.Func([HttpRequest], [HttpResponse], ['query']),
    'logo' : IDL.Func([], [IDL.Vec(IDL.Nat8)], ['query']),
    'meta' : IDL.Func([], [CandidTokenMetadata], ['query']),
    'mint' : IDL.Func(
        [IDL.Text, IDL.Nat, IDL.Opt(IDL.Nat64)],
        [OperationResult],
        [],
      ),
    'name' : IDL.Func([], [IDL.Text], ['query']),
    'owner' : IDL.Func([], [IDL.Principal], ['query']),
    'setDesc' : IDL.Func(
        [IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))],
        [BooleanResult],
        [],
      ),
    'setFee' : IDL.Func(
        [CandidTokenFee, IDL.Opt(IDL.Nat64)],
        [BooleanResult],
        [],
      ),
    'setFeeTo' : IDL.Func([IDL.Text, IDL.Opt(IDL.Nat64)], [BooleanResult], []),
    'setLogo' : IDL.Func([IDL.Opt(IDL.Vec(IDL.Nat8))], [BooleanResult], []),
    'setOwner' : IDL.Func(
        [IDL.Principal, IDL.Opt(IDL.Nat64)],
        [BooleanResult],
        [],
      ),
    'symbol' : IDL.Func([], [IDL.Text], ['query']),
    'tokenInfo' : IDL.Func([], [TokenInfo], ['query']),
    'totalSupply' : IDL.Func([], [IDL.Nat], ['query']),
    'transfer' : IDL.Func(
        [IDL.Opt(IDL.Vec(IDL.Nat8)), IDL.Text, IDL.Nat, IDL.Opt(IDL.Nat64)],
        [OperationResult],
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
        [OperationResult],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  const CandidTokenFee = IDL.Record({
    'rate' : IDL.Nat32,
    'minimum' : IDL.Nat,
    'rateDecimals' : IDL.Nat8,
  });
  const ArchiveOptions = IDL.Record({
    'num_blocks_to_archive' : IDL.Nat32,
    'trigger_threshold' : IDL.Nat32,
    'max_message_size_bytes' : IDL.Opt(IDL.Nat32),
    'cycles_for_archive_creation' : IDL.Opt(IDL.Nat64),
    'node_max_memory_size_bytes' : IDL.Opt(IDL.Nat32),
  });
  return [
    IDL.Opt(IDL.Vec(IDL.Nat8)),
    IDL.Opt(IDL.Vec(IDL.Nat8)),
    IDL.Text,
    IDL.Text,
    IDL.Nat8,
    IDL.Nat,
    CandidTokenFee,
    IDL.Opt(IDL.Principal),
    IDL.Opt(ArchiveOptions),
  ];
};

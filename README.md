# Dfinity Fungible Token Standard

## Overview

Thinking in [Dfinity Fungible Token Standard](./Thinking-in-dft.md)

## How to create a fungible token with 1 line of code

```RUST
dft_derive::standard_basic!();
```

## Tools

[DFT issuance tool](https://github.com/Deland-Labs/dft-issuance-tool)

## Standard

```RUST
type CallData = record { method : text; args : vec nat8 };
type Fee = record { rate : nat; lowest : nat };
type KeyValuePair = record { k : text; v : text };
type MetaData = record { fee : Fee; decimals : nat8; name : text; total_supply : nat; symbol : text; };
type TxRecordsResult = variant { Ok : vec TxRecord; Err : text };
type TxRecordResult = variant { Ok : TxRecord; Err : text; Forward : principal; };
type TxRecord = variant {
  Approve : record { nat; principal; TokenHolder; TokenHolder; nat; nat; nat64; };
  Burn : record { nat; principal; TokenHolder; nat; nat64 };
  Transfer : record { nat; principal; TokenHolder; TokenHolder; nat; nat; nat64; };
};
//DFT support AccountId (ICP holder address) and Principal as token holder
type TokenHolder = variant { Account : text; Principal : principal };
type TokenInfo = record {
  allowance_size : nat;
  fee_to : TokenHolder;
  owner : principal;
  cycles : nat64;
  tx_count : nat;
  holders : nat;
  storages : vec principal;
};
type TransactionResponse = record { txid : text; error : opt vec text };
type TransactionResult = variant { Ok : TransactionResponse; Err : text };

service : {
  // Return token's name
  name : () -> (text) query;

  // Return token's symbol
  symbol : () -> (text) query;

  // Return token's decimals
  decimals : () -> (nat8) query;

  // Return token's totalSupply
  totalSupply : () -> (nat) query;

  // Return token's fee setting
  fee : () -> (Fee) query;

  // Set token's fee setting
  setFee : (Fee) -> (bool);

  // Any fee will send to the feeHolder
  setFeeTo : (feeHolder: text) -> (bool);

  // Return all of the meta data of a token.
  meta: () -> (MetaData) query;

  // Return all of the extend data of a token.
  // Extend data show more information about the token
  // supported keys:
  //   OFFICIAL_SITE, OFFICIAL_EMAIL, DESCRIPTION, WHITE_PAPER
  //   MEDIUM, BLOG, REDDIT, SLACK, FACEBOOK, TWITTER,
  //   GITHUB, TEGEGRAM, WECHAT, LINKEDIN, DISCORD,
  //   DSCVR, OPENCHAT, DISTRIKT, WEACT

  extend: () -> (vec KeyValuePair) query;

  // Set extend data of a token.
  setExtend : (vec KeyValuePair) -> (bool);

  // Return token logo picture
  logo : () -> (vec nat8) query;

  // Set the logo of a token
  setLogo : (logo : vec nat8) -> (bool);

  // Returns the account balance of another account with address owner.
  balanceOf: (holder: text) -> (nat) query;

  // Return token's owner
  owner : () -> (principal);

  // Set the token owner
  //    owner can invoke [setFee, setFeeTo, setLogo, setExtend, setOwner]
  setOwner : (owner: principal) -> (bool);

  // Return token's info:
  // owner: OWNER,
  //          holders: total holder count of the token,
  //          allowance_size: total allowance count of the token,
  //          fee_to: feeHolder,
  //          tx_count: total transaction count of the token,
  //          cycles: cycles balance of the token
  //          storages : auto-scaling storage canister ids
  tokenInfo : () -> (TokenInfo) query;

  // Returns the amount which spender is still allowed to withdraw from owner.
  allowance:(owner: text, spender: text)->(nat) query;

  // Allows `spender` to withdraw from your account multiple times, up to the `value` amount.
  // If this function is called again it overwrites the current allowance with value.
  // If `calldata` is not null and `spender` is canister, approve means approveAndCall.
  approve: (fromSubAccount: opt vec nat8, spender: text, value: nat, calldata: opt CallData) -> (TransactionResult);

  //Get all allownances of the holder
  allowancesOf : (holder: text) -> (vec record { TokenHolder; nat }) query;

  // Transfers value amount of tokens from `address from` to [address to].
  // The transferFrom method is used for a withdraw workflow, allowing canister
  // to transfer tokens on your behalf.
  // If the receiver's (`to`) notification hook function exists,  will be called.
  transferFrom: (spenderSubAccount: opt vec nat8, from: text, to: text,value: nat) ->(TransactionResult);

  // Transfer from :
  //      1. `fromSubAccount` is not null : Use the accountId generated based on the caller's Principal and the provided `fromSubAccount`
  //      2. `fromSubAccount` is  null    : Use caller's Principal
  // Then transfer `value` from the holder to `to` , return TransferResponse
  // `to` can be an AccountId , a Principal,or a canister id (If the container has a notification hook function, a notification will be triggered).
  // If `calldata` is not null and `to` is canister, transfer means transferAndCall.
  // Transfer 0 value ​​will be reject.
  transfer: (fromSubAccount:opt vec nat8, to: text, value: nat, calldata: opt CallData) -> (TransactionResult);

  // Get last transcation of the DFT, max size is 200
  lastTransactions : (size: nat64) -> (TxRecordsResult) query;

  // Get transcation information by id
  // If not exist in DFT, return the storage canister id of the transaction located
  // Call transactionById to the storage canister id again ,will return the transaction information.
  transactionById : (transactionId: text) -> (TxRecordResult) query;

  // Get transcation information by tx index
  // If not exist in DFT, return the storage canister id of the transaction located
  // Call transactionById to the storage canister id again ,will return the transaction information.
  transactionByIndex : (nat) -> (TxRecordResult) query;
}
```

## Auto-Scaling Storage (ATSS) details

1. When will the ATSS be created?

   - Create the first ATSS when the DFT's transactions (txs) > 2000. It means that no auto-scaling storage will be created before the DFT's txs > 2000 to save cycles

   - Create the next ATSS when the current ATSS's storage size is not enough to store 1000 txs.

2. What's the fallback strategy?
   If the creation of the ATSS fails, the txs will be stored in the DFT, txs will be moved to ATSS when the creation is successful.
   Possible reasons for failure:
   - Not enough cycles balance to create ATSS.
   - Other unknown reason.

## Compile dependencies

### dfx

```bash
sh -ci "$(curl -fsSL https://sdk.dfinity.org/install.sh)"
```

### rust

Linux & Mac

1. Install Rust & cmake & optimizer

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
brew install cmake
cargo install ic-cdk-optimizer
```

2. Add wasm32-unknown-unknown target

```bash
rustup target add wasm32-unknown-unknown
```

## How to test?

```bash
   make test_rs
   make test_motoko
```

## About us

We are from Deland-Labs team.

We are building a decentralized exchange based on Dfinity with Open Order Protocol.

Offcial Website : [https://delandlabs.com](https://delandlabs.com)

## References

-[1] [Dfinity Developer Center: Canister interface](https://sdk.dfinity.org/docs/interface-spec/index.html#system-api-imports)

-[2] [Dfinity Forum: thoughts-on-the-token-standard](https://forum.dfinity.org/t/thoughts-on-the-token-standard/4694)

-[3] [Toniq-Labs: ic-fungible-token](https://github.com/Toniq-Labs/ic-fungible-token)

-[4] [SuddenlyHazel: Token-Standard](https://github.com/SuddenlyHazel/token-standard/pull/1)

-[5] [Dfinance-tech: ic-token](https://github.com/dfinance-tech/ic-token)

-[6] [Plug: Token-Standard](https://github.com/Psychedelic/standards)

-[7] [Ethereum: EIPS-EIP20 & EIP667 & EIP777 & EIP1820](https://github.com/ethereum/EIPs)

-[8] [Candid](https://github.com/dfinity/candid/)

-[9] [Why are ERC20 allowances necessary?](https://kalis.me/unlimited-erc20-allowances/)

-[10] [sudograph](https://github.com/sudograph/sudograph)

-[11] [Dfinity Self Describing Standard](https://github.com/Deland-Labs/dfinity-self-describing-standard)

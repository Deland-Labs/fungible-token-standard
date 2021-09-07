# Dfinity Fungible Token Standard

## Overview

  Thinking in [Dfinity Fungible Token Standard](./Thinking-in-dft.md)

## Tools 
  [DFT issuance tool](https://github.com/Deland-Labs/dft-issuance-tool)

## Standard
```RUST
type ApproveResult = variant { Ok : opt String; Err : String };
type BurnResult = variant { Ok; Err : String };
type CallData = record { method : text; args : vec nat8 };
type Fee = record { lowest: nat; rate :nat32 };
type KeyValuePair = record { k : text; v : text };
type MetaData = record {
  fee : Fee;
  decimals : nat8;
  name : text;
  total_supply : nat;
  symbol : text;
};
//DFT support AccountId (ICP holder address) and Principal as token holder
type TokenHolder = variant { Account : text; Principal : principal; };
type TransferResult = variant {
  Ok : record { nat; opt vec String };
  Err : String;
};
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

  // Return token logo picture
  logo : () -> (vec nat8) query;

  // Returns the account balance of another account with address owner.
  balanceOf: (holder: text) -> (nat) query;

  // Returns the amount which spender is still allowed to withdraw from owner.
  allowance:(owner: text, spender: text)->(nat) query;

  // Allows `spender` to withdraw from your account multiple times, up to the `value` amount. 
  // If this function is called again it overwrites the current allowance with value.
  // If `calldata` is not null and `spender` is canister, approve means approveAndCall.
  approve: (fromSubAccount: opt vec nat8, spender: text, value: nat, calldata: opt CallData) -> (ApproveResult);
  // Transfers value amount of tokens from `address from` to [address to].
  // The transferFrom method is used for a withdraw workflow, allowing canister
  // to transfer tokens on your behalf.
  // If the receiver's (`to`) notification hook function exists,  will be called.
  transferFrom: (spenderSubAccount: opt vec nat8, from: text, to: text,value: nat) ->(TransferResult);

  // Transfer from : 
  //      1. `fromSubAccount` is not null : Use the accountId generated based on the caller's Principal and the provided `fromSubAccount`
  //      2. `fromSubAccount` is  null    : Use caller's Principal
  // Then transfer `value` from the holder to `to` , return TransferResponse
  // `to` can be an AccountId , a Principal,or a canister id (If the container has a notification hook function, a notification will be triggered).
  // If `calldata` is not null and `to` is canister, transfer means transferAndCall.
  // Transfer 0 value ​​will be reject.
  transfer: (fromSubAccount:opt vec nat8, to: text, value: nat, calldata: opt CallData) -> (TransferResult);

  // Destroys `amount` tokens from `account`, reducing the total supply.
  burn: (fromSubAccount: opt vec nat8,amount: nat) -> (BurnResult);
}
```

## How to test?
```bash
   make test_rs
   make test_motoko
```

## About us

   We are from Deland-Labs team. 

   We are building a decentralized exchange based on Dfinity with Open Order Protocol.

   Offcial Website : [https://deland.one](https://deland.one)


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

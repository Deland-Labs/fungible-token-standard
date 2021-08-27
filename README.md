# Dfinity Fungible Token Standard

## Overview

Token standard will help Dfinity ecological developers adopt the same standard and promote the prosperity of the Dfinity ecosystem

## Rules of Token Standard Design

ERC20 is the first token standard in the blockchain world, and it has been fully verified and recognized. Therefore, when designing the [Dfinity Fungible Token Standard], it is necessary to refer to the existing ERC20 standard.

At the same time, the formulation of [Dfinity Fungible Token Standard] should meet the following goals:

1. Improving the ERC20 standard

2. Being suitable for Dfinity

## Improve ERC20 standard

### How to improve ERC20

ERC20 was created in the early days of Ethereum. In the process of Eth ecological development, the developer found that ERC20 was not perfect, so the developer designed the ERC223\ERC667\ERC777 standard in an attempt to improve ERC20. We will refer to these standards and develop a one that combines the advantages of these standards.

1. ERC223 tries to solve that the ERC20 transfer recipient (contract) does not support the transfer of Token, the transferred Token will be lost (similar to sending Token to a black hole address)

   Solution details: Fallback processing method, the recipient (contract) must implement the tokenFallback method to determine whether the recipient supports ERC20 Token

2. ERC667 adds transferAndCall, in order to realize the simultaneous execution of transfer and call, and solve similar problems with ERC223

3. ERC777 uses send as the standard sending method, controls whether the transfer is accepted or not through the hook function, and uses the operator authorization method to replace the approve method as a proxy transfer solution

With reference to the above standards, we have the following considerations:

1. ERC667 and ERC223 solve similar problems, so just keep one of them

2. ERC777 send VS ERC20 transfer is to realize the transfer. Which plan do you choose to keep?

   ERC20 transfer does not contain other logic besides the transfer;

   ERC777 send contains transfer and:

   - During the transfer process, if the sender implements the tokenToSend hook function, the function will be called to accept or reject the transfer before the transfer

   - During the transfer process, if the transfer receiver implements the tokensReceived hook function, the function will be called after the transfer to accept or reject the transfer

ERC777 implements the capabilities that ERC20 does not have, allowing the sender/receiver to control whether to accept the transfer. It seems more reasonable to use ERC777 send method. ERC20 is more popular, so the ERC777 scheme is adopted, but using transfer as the method name is easier for ERC20 users to accept.

The implementation of ERC777 relies on the ERC1820 registration contract to register the sender/receiver hook function, so no matter the sender and receiver are ordinary addresses, even the contract address can register hook functions. (This topic will be discussed again in the <strong>[Suitable for Dfinity]</strong> section below)

3. The hook function of the ERC777 receiver realizes a function similar to ERC667, so the function coverage of ERC667 can be completed by adopting the ERC777 standard

4. Operator authorization solution of ERC777 VS ERC20 approve solution

   The operator authorization scheme of ERC777 does not limit the allowance of authorization, and the management granularity is bigger. ERC20 Approve can not only meet the needs of the ERC777 authorization program, but also through the approval allowance Approve program seems to be a more reasonable choice, which can control the credit range available to everyone and achieve more refined management than ERC777

5. ERC777 provides a default precision value of 18 for the token, and supports setting the minimum step unit for tokens.

- Different precision support is more suitable for the needs of different scenes, and the design of keeping decimals seems to be a more reasonable choice

- ERC777 non-granular integer operations will be reverted, which will increase the frequency of abnormal user calls, so this design is abandoned

### Improved standards

Based on the above considerations, the improved draft standard is as follows:

```
service: {
  name: () -> (text) query;
  symbol: () -> (text) query;
  decimals: () -> (nat64) query;
  totalSupply: () -> (nat64) query;

  balanceOf: (owner: principal) -> (nat64) query;
  allowance: (owner: principal, spender: principal) -> (nat64) query;
  approve: (spender: principal, value: nat64) -> (bool);
  transferFrom: (sender: principal, receiver: principal, value: nat64) -> (bool);
  send: (receiver: principal, value: nat64, args:opt vec nat8) -> (bool);
}
```

## Suitable for Dfinity

### Problems to be solved

The design of Token Standard should fully consider the difference between Dfinity and Ethereum, and clarify the problems to be solved:

1. No atomicity similar to EVM cross-contract calls

- Conclusion: It is necessary to refactor the interface；

2. No built-in EVENT support

- Probelm: Historical content such as transaction records needs to be separately for storage

- Consideration: On Forum, there are two ideas (Pubsub/Notify)

  When the Token is transferred, Notify informs the recipient, which can fill the missing EVENT.

  When the Token recipient not a canister, which means can not notify, it is necessary to support query transaction records.

  Token does not have sufficient reason to implement Pubsub to satisfy third parties irrelevant to actual operations

- Conclusion: Notify is a better way; should support query transaction history;

3. Built-in storage support, can store more data content

- Problem: The current storage limit is 4G, which can store more content cheaply, but storage expansion needs to be considered

- consider:

  tx history, should be stored separately to avoid storage limitations

  Built-in storage can support Token to store more self-describing information

- Conclusion:

  Separate storage of transaction history
  Token implements self-description

4. The call of the contract does not require the caller to pay gas fees (the contract publisher provides gas fees in the contract)

- Problem: Need to consider the cost of DDOS attacks that call the contract

- Conclusion: The charging logic should be designed in the Token

5. There are two different identities in Dfinity, Internet Identity (II for short) and Principal ID

- Problem: which identity to use as the choice of token standard is an important question

- Consideration: Dfinity's II is an implementation of DID, although DID is based on Principal ID

- Conclusion: It is necessary for the Token standard to be compatible with different identities, in order to meet the needs of different identity scenarios

6. No black hole address

- Question: If there is a need to destroy Token, how to deal with it?

- Conclusion: The burn interface should be designed in the Token standard

7. approve/transferFrom (Approve is a pairing function for TransferFrom) keep or remove

- Question: Whether approve/transferFrom is removed is controversial in the Forum discussion

- consider:

  approve/transferFrom appears in ERC20 mainly because:

  > Using Ethereum's native ETH token, you can call the smart contract function and send ETH to the contract at the same time. This is done using payable. But because the ERC20 token itself is a smart contract, it is not possible to directly send the token to the smart contract while calling one of its functions; therefore, the ERC20 standard allows smart contracts to transfer tokens on behalf of the user-using the transferFrom() function. For this, users need to allow smart contracts to transfer these tokens on their behalf

  However, in the Dex and lending scenarios of Ethereum, Approve is often accompanied by the possibility of simultaneous operation of two tokens. Approve can avoid the repeated payment problem which transaction brought about, has a good supplementary use scenario for transfer.

- Conclusion: Approve/transferFrom should be supported

8. TransferAndCall vs Receiver Notify

- Probelm: which option is more suitable

- consider:

Notify can meet the basic notification needs. Although it cannot support better flexibility, it is sufficient to meet the transfer scenario

TransferAndCall provides better flexibility, but it depends on the transfer caller to fully understand the method and parameters corresponding to the call, which is not needed for most transfer scenarios

- Conclusion: Both are supported at the same time, integrated in the transfer function

<s>If the user specifies the call (specify the target method and parameters), only the call will be executed, and the notification will not be executed;</s>

<s>If the user does not specify the call (specify the target method and parameters), only execute Notify;</s>

Token standard should execute Notify first, and then execute call;

9. approveAndCall VS transferAndCall

- Problem: Some developers support approveAndCall, so we compare it with transferAndCall. Due to problem 1 (atomic problem), approveAndCall and transferAndCall are two sets of non-atomic operations, and there is no difference in essence.

- Consideration: In some scenarios, when multiple Tokens need to be transferred at the same time, transferAndCall can not meet such needs. After approval, execute transferFrom in the final call to pay multiple tokens at once

- Conclusion: Support approveAndCall and transferAndCall to meet the flexible needs of more scenarios.

### What does Dfinity Fungible Token Standard need to achieve?

<br>

1. Interface self-description

Dfinity needs to provide a common contract interface registration/query service similar to ERC1820.

Dfinity currently does not have such a service, but because of <strong>[problems to be solved]</strong> economic considerations, no one wants to build such a service.

Dfinity can solve the problem solved by ERC1820 through [Dfinity Self Describing Standard](https://github.com/Deland-Labs/dfinity-self-describing-standard)

2. Information self-describing

Etherscan, MyEthereumWallet, Imtoken, TokenPocket, Dapp all have more information requirements for ERC20, such as Logo, introduction, white paper, social media, official website, contact information, etc. Each place that needs this information needs to be maintained independently, so information appears Inconsistent. It is necessary to solve this problem through the design of <strong>[Dfinity Fungible Token Standard]</strong>

Based on the above problems and requirements, combined with the ERC standard formed in the previous step, the following draft standards are formulated:

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
  // OFFICIAL_SITE
  // MEDIUM
  // OFFICIAL_EMAIL
  // DESCRIPTION
  // BLOG
  // REDDIT
  // SLACK
  // FACEBOOK
  // TWITTER
  // GITHUB
  // TEGEGRAM
  // WECHAT
  // LINKEDIN
  // DISCORD
  // WHITE_PAPER
  extend: () -> (vec KeyValuePair) query;

  // Return token logo picture
  logo : () -> (vec nat8) query;

  // Returns the account balance of another account with address owner.
  balanceOf: (holder: text) -> (nat) query;

  // Returns the amount which spender is still allowed to withdraw from owner.
  allowance:(owner: text, spender: text)->(nat) query;

  // Allows spender to withdraw from your account multiple times, up to the value amount. If this function is called again it overwrites the current allowance with value.
  // If calldata is not empty, approveAndCall will be executed.
  approve: (fromSubAccount: opt vec nat8, spender: text, value: nat, calldata: opt CallData) -> (ApproveResult);
  // Transfers value amount of tokens from [address from] to [address to].
  // The transferFrom method is used for a withdraw workflow, allowing canister
  // to transfer tokens on your behalf.
  transferFrom: (spenderSubAccount: opt vec nat8, from: text, to: text,value: nat) ->(TransferResult);

  // receiver's Notify hood function if exist.
  // Transfers of 0 values ​​will be reject.
  // Generates an AccountIdentifier based on the caller's Principal and
  // the provided SubAccount*, and then attempts to transfer amount from the
  // generated AccountIdentifier to recipient, and returns the outcome as TransferResponse.
  // recipient can be an AccountIdentitifer, a Principal (which then transfers to the default subaccount),
  // or a canister (where a callback is triggered).
  // calldata means transferAndCall
  transfer: (fromSubAccount:opt vec nat8,to: text, value: nat, calldata: opt CallData) -> (TransferResult);

  // Destroys `amount` tokens from `account`, reducing the total supply.
  burn: (fromSubAccount: opt vec nat8,amount: nat) -> (BurnResult);

  // Return if canister support interface, for example: supportedInterface("balanceOf:(text)->(nat)")
  // Implement [Dfinity Self Describing Standard](https://github.com/Deland-Labs/dfinity-self-describing-standard)
  supportedInterface : (text) -> (bool) query;

  // get cycles balance in token canister
  cyclesBalance : () -> (nat) query;
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

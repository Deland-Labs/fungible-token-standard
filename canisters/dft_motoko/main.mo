/**
 * Module     : token.rs
 * Copyright  : 2021 Deland-Labs Team
 * License    : Apache 2.0 with LLVM Exception
 * Maintainer : Deland Team (https://deland.one)
 * Stability  : Experimental
 */
import Prim "mo:⛔";
import HashMap "mo:base/HashMap";
import Principal "mo:base/Principal";
import Debug "mo:base/Debug";
import Types "./types";
import Time "mo:base/Time";
import Iter "mo:base/Iter";
import Array "mo:base/Array";
import Option "mo:base/Option";
import Error "mo:base/Error";
import Text "mo:base/Text";
import Nat8 "mo:base/Nat8";
import Nat "mo:base/Nat";
import Char "mo:base/Char";
import List "mo:base/List";
import Result "mo:base/Result";
import ExperimentalCycles "mo:base/ExperimentalCycles";
import PrincipalExt "./utils/PrincipalExt";
import AID "./utils/AccountIdentifier";

shared(msg) actor class Token(name: Text, symbol: Text, decimals: Nat8, totalSupply: Nat){
    type TransactionId = Types.TransactionId;
    type TokenHolder = Types.TokenHolder;
    type MetaData = Types.MetaData;
    type CallData = Types.CallData;
    type TxRecord = Types.TxRecord;
    type ApproveResult = Types.ApproveResult;
    type TransferResult = Types.TransferResult;

    type StorageActor = actor {
        graphql_query_custom: query (Text, Text) -> async (Text);
        graphql_mutation: (Text, Text) -> async (Text);
    };

    private stable var _owner : Principal = msg.caller;
    private stable var _name : Text = name;
    private stable var _symbol : Text = symbol;
    private stable var _decimals : Nat8 = decimals;
    private stable var _totalSupply : Nat = totalSupply;
    private stable var _fee : Types.Fee = { lowest = 0 ; rate = 0 ; } ;
    private stable var _txIdCursor : Nat = 0 ;

    private stable var _logo: [Nat8] = [];
    private stable var _feeTo : TokenHolder = #Principal(msg.caller) ;
    private stable var _storageCanisterID : ?Principal = null;


    private stable var _extendDataEntries : [(Text, Text)] = [];
    private stable var _balanceEntries : [(TokenHolder, Nat)] = [];
    private stable var _allowanceEntries : [(TokenHolder, [(TokenHolder, Nat)])] = [];

    private var _balances = HashMap.HashMap<TokenHolder, Nat>(1, Types.TokenHolder.equal, Types.TokenHolder.hash);
    private var _allowances = HashMap.HashMap<TokenHolder, HashMap.HashMap<Types.TokenHolder, Nat>>(1, Types.TokenHolder.equal, Types.TokenHolder.hash);
    private var _extendDatas = HashMap.HashMap<Text, Text>(1, Text.equal, Text.hash);

    private stable var storageCanister : ?StorageActor = null;
    
    private let MSG_ONLY_OWNER= "DFT: caller is not the owner";
    private let MSG_INVALID_SPENDER = "DFT: invalid spender";
    private let MSG_INVALID_FROM = "DFT: invalid format [from]";
    private let MSG_INVALID_TO= "DFT: invalid format [to]";
    private let MSG_FAILED_TO_CHARGE_FEE = "DFT: Failed to charge fee - insufficient balance";
    private let MSG_ALLOWANCE_EXCEEDS = "DFT: transfer amount exceeds allowance";
    private let MSG_BALANCE_EXCEEDS = "DFT: transfer amount exceeds balance";
   
    private let DECIMALS_FEE_RATE: Nat = 8;
    private let TX_TYPES_APPROVE: Text = "approve";
    private let TX_TYPES_TRANSFER: Text = "transfer";
    private let TX_TYPES_BURN: Text = "burn";


    _balances.put(#Principal(_owner), totalSupply);

    public query func meta() : async MetaData {
      return {
        name = _name;
        symbol = _symbol;
        decimals = _decimals;
        total_supply = _totalSupply;
        fee = _fee;
      };
    };

    public query func extend() : async [(Text,Text)] {
      return Iter.toArray(_extendDatas.entries());
    };

    public shared(msg) func updateExtend( extendDatas: [(Text,Text)]) : async Bool {
      if ( _owner != msg.caller ) { throw Error.reject(MSG_ONLY_OWNER); };
      
      for ((k , v) in extendDatas.vals()) {
        if (Types.ExtendData.isValidKey(k)){
           _extendDatas.put( k , v );
        }
      };

      return true;
    };

    public query func logo() : async [Nat8] {
      return _logo;
    };

    public shared(msg) func updateLogo( logo: [Nat8]) : async Bool {
      if ( _owner != msg.caller ) { throw Error.reject(MSG_ONLY_OWNER); };      
      _logo := logo;
      return true;
    };

    public query func balanceOf(owner: Text) : async Nat {
      let holder = Types.TokenHolder.fromText(owner);
      if (Option.isNull(holder)) { return 0; }
      else { return _balanceOf(Option.unwrap(holder)) };
    };

    private func _balanceOf(holder: TokenHolder) : Nat {
      switch (_balances.get(holder)) {
        case (?balance) { return balance; };
        case (_) { return 0; };
      }
    };

    public query func allowance(owner: Text, spender: Text) : async Nat {
      let ownerHolder = Types.TokenHolder.fromText(owner);
      if (Option.isNull(ownerHolder)) return 0;

      let spenderHolder = Types.TokenHolder.fromText(spender);
      if (Option.isNull(spenderHolder)) return 0;
        
      return _allowance(Option.unwrap(ownerHolder), Option.unwrap(spenderHolder));
    };

    private func _allowance(owner: TokenHolder, spender: TokenHolder) : Nat {
      switch(_allowances.get(owner)) {
        case (?allowanceOwner) {
          switch(allowanceOwner.get(spender)) {
            case (?allowance) { return allowance; };
            case (_) { return 0; };
          }
        };
        case (_) { return 0; };
      }
    };

    public shared(msg) func approve(subAccount: ?AID.Subaccount, spender: Text, value: Nat, callData: ?CallData) : async ApproveResult {
      var ownerHolderParseResult = ?Types.TokenHolder.fromPrincipal(msg.caller);
      if ( Option.isSome(subAccount)) {
        let aid =AID.fromPrincipal(msg.caller, subAccount);
        ownerHolderParseResult := ?Types.TokenHolder.fromAid(aid);
      };
      assert(Option.isSome( ownerHolderParseResult )) ;

      let spenderHolderParseResult = Types.TokenHolder.fromText(spender);
      if (Option.isNull( spenderHolderParseResult )) throw Error.reject(MSG_INVALID_SPENDER);
      let ownerHolder = Option.unwrap(ownerHolderParseResult);
      let spenderHolder = Option.unwrap(spenderHolderParseResult);

      let allowanceOwnerResult = _allowances.get(ownerHolder);
      let approveFee = _calcApproveFee ();
      let chargeFeeResult = _chargeApproveFee(ownerHolder, approveFee);

      switch chargeFeeResult {
        case (#ok(_)) {  };
        case (#err(emsg)) { return #Err(emsg) };
      };

      if (value == 0 and Option.isSome(allowanceOwnerResult)) {
        let allowanceOwner =  Option.unwrap(allowanceOwnerResult) ;
        allowanceOwner.delete(spenderHolder);
        if (allowanceOwner.size() == 0) { _allowances.delete(ownerHolder); }
        else { _allowances.put(ownerHolder, allowanceOwner); };
      } 
      else if (value != 0 and Option.isNull(_allowances.get(ownerHolder))) {
          var tempAllowance = HashMap.HashMap<TokenHolder, Nat>(1, Types.TokenHolder.equal, Types.TokenHolder.hash);
          tempAllowance.put(spenderHolder, value);
          _allowances.put(ownerHolder, tempAllowance);
      } 
      else if (value != 0 and Option.isSome(_allowances.get(ownerHolder))) {
          let allowanceOwner = Option.unwrap(_allowances.get(ownerHolder));
          allowanceOwner.put(spenderHolder, value);
          _allowances.put(ownerHolder, allowanceOwner);
      };

      ignore  _saveTxRecordToGraphql(#Approve{
         owner = ownerHolder;
         spender = spenderHolder;
         value = value;
         fee = approveFee;
         timestamp = Time.now();
      });

      if(Option.isSome(callData)){
        let callRes = await _executeCall(spenderHolder, Option.unwrap(callData));
        switch (callRes){
          case (#ok(_)) {};
          case (#err(emsg)) { return #Ok(?emsg); };
        };
      };
      return #Ok(null);
    };
   
    public shared(msg) func transferFrom(subAccount: ?AID.Subaccount, from: Text, to: Text, value: Nat) : async TransferResult {
      var spenderHolderParseResult = ?Types.TokenHolder.fromPrincipal(msg.caller);
      if ( Option.isSome(subAccount)) {
        let aid =AID.fromPrincipal(msg.caller, subAccount);
        spenderHolderParseResult := ?Types.TokenHolder.fromAid(aid);
      };
     
      let fromHolderParseResult = Types.TokenHolder.fromText(from);
      if (Option.isNull( fromHolderParseResult )) return #Err(MSG_INVALID_FROM);
      let toHolderParseResult = Types.TokenHolder.fromText(to);
      if (Option.isNull( toHolderParseResult )) return #Err(MSG_INVALID_TO);

      let fromHolder = Option.unwrap(fromHolderParseResult) ;
      let spenderHolder = Option.unwrap(spenderHolderParseResult) ;
      let toHolder = Option.unwrap(toHolderParseResult) ;
      let spenderAllowance = _allowance(fromHolder, spenderHolder);
      let fee = _calcTransferFee(value);

      if (spenderAllowance < value + fee) return #Err(MSG_ALLOWANCE_EXCEEDS);
     
      let newAllowance : Nat = spenderAllowance - (value + fee);
      let allowanceFrom = Option.unwrap(_allowances.get(fromHolder));
      if (newAllowance != 0) {
        allowanceFrom.put(spenderHolder, newAllowance);
        _allowances.put(fromHolder, allowanceFrom);
      }
      else { 
        allowanceFrom.delete(spenderHolder);
        if (allowanceFrom.size() == 0) { _allowances.delete(fromHolder); }
        else { _allowances.put(fromHolder, allowanceFrom); }; 
      };
      return await _trasfer(fromHolder, toHolder, value);
    };
    
    public shared(msg) func transfer(subAccount: ?AID.Subaccount, to: Text, value: Nat, callData: ?CallData) : async TransferResult {
      var fromHolderParseResult = ?Types.TokenHolder.fromPrincipal(msg.caller);
      if ( Option.isSome(subAccount)) {
        let aid =AID.fromPrincipal(msg.caller, subAccount);
        fromHolderParseResult := ?Types.TokenHolder.fromAid(aid);
      };
    
      let toHolderParseResult = Types.TokenHolder.fromText(to);
      if (Option.isNull( toHolderParseResult )) return #Err(MSG_INVALID_TO);

      let fromHolder = Option.unwrap(fromHolderParseResult) ;
      let toHolder = Option.unwrap(toHolderParseResult) ;
     
      let transferRes = await _trasfer(fromHolder, toHolder, value);
      
      switch (transferRes) {
        case (#Ok{txid : TransactionId; error : ?[Text]; }) {
          var errors = List.nil<Text>();
          if(Option.isSome(error)) errors := List.fromArray(Option.unwrap(error));
          if(Option.isSome(callData)){
            let callRes = await _executeCall(toHolder, Option.unwrap(callData));
            switch (callRes){
                case (#ok(_)) {};
                case (#err(emsg)) { errors := List.push(emsg, errors); };
            };
          };
          if (List.size(errors) > 0) { return #Ok { txid = txid; error = ?List.toArray<Text>(errors); }; }
          else { return #Ok{txid = txid; error = null}; };
        };
        case (#Err(emsg)) { return #Err(emsg) };
      };
    };

    private func _trasfer(from: TokenHolder, to: TokenHolder, value: Nat) : async TransferResult {
      let fee = _calcTransferFee(value);
      let fromBalance = _balanceOf(from);
      if (fromBalance < value + fee ) return #Err(MSG_BALANCE_EXCEEDS);

      // before transfer
      let beforeSendingCheckResult = _onTokenSending(from, to, value);

      switch (beforeSendingCheckResult){
        case (#ok()) {};
        case (#err(emsg)) { return #Err(emsg); };
      };

      let fromBalanceNew : Nat =  fromBalance - value - fee;
      if (fromBalanceNew != 0) { _balances.put(from, fromBalanceNew); }
      else { _balances.delete(from); };

      let toBalance = _balanceOf(to);
      let toBalanceNew =  toBalance + value;

      if (toBalanceNew != 0) { _balances.put(to, toBalanceNew); }
      else { _balances.delete(to); };

      _settleFee(fee);

      let txId = await _saveTxRecordToGraphql(#Transfer{
         from = from;
         to = to;
         value = value;
         fee = fee;
         timestamp = Time.now();
      });
      let afterTokenSendNotifyRes = await _onTokenReceived(from, to , value);
      switch (afterTokenSendNotifyRes){
        case (#ok(_)) {};
        case (#err(emsg)) { return #Ok({ txid = txId; error = ?Array.make(emsg); }); };
      };
      return #Ok({ txid = txId; error = null; });
    };
    
    // TODO:Can not perform call in a generic way,I am looking for a solution
    // https://github.com/dfinity/motoko/issues/2703 when motoko support, we will imple out executeCall
    private func _executeCall(receiver: TokenHolder, callData: CallData) : async Result.Result<Bool, Text> {
      switch(receiver) {
        case (#Account accountID) {return #ok(true); };
        case (#Principal principal) {
          if(PrincipalExt.isCanister(principal)){
             return #ok(true);
          }
          else return #ok(true);
        };
      };       
    };

    private func _calcApproveFee() : Nat {
      return _fee.lowest;
    };

    private func _calcTransferFee(value: Nat) : Nat {
      return Nat.max(_fee.lowest, value * _fee.rate/ Nat.pow(10, DECIMALS_FEE_RATE) );
    };

    private func _chargeApproveFee(payer: TokenHolder, fee: Nat) : Result.Result<Bool, Text> {
      if (fee == 0) {
        return #ok(true);
      };

      let payerBalance = _balanceOf( payer );
      
      if (payerBalance < fee) {
        return #err(MSG_FAILED_TO_CHARGE_FEE);
      };

      _balances.put(payer, payerBalance - fee );
      _settleFee(fee);
      return #ok(true);
    };

    private func _settleFee(fee: Nat) {
      if (fee > 0) {
        _balances.put(_feeTo, fee);
      };
    };

    private func _saveTxRecordToGraphql(tx: TxRecord) : async TransactionId {
      //TODO: imple save tx to graphql
      return 0;
    };

    private func _onTokenReceived(from: TokenHolder, to: TokenHolder, value: Nat) : async Result.Result<Bool, Text> {
      return #ok(true);
    };


    // do something becore sending
    private func _onTokenSending(from: TokenHolder, to: TokenHolder, value: Nat) : Result.Result<(), Text> 
    {
      #ok(());
    };
};
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
import Int "mo:base/Int";
import Char "mo:base/Char";
import List "mo:base/List";
import Result "mo:base/Result";
import ExperimentalCycles "mo:base/ExperimentalCycles";
import PrincipalExt "./utils/PrincipalExt";
import AID "./utils/AccountIdentifier";

shared(msg) actor class Token(name_: Text, symbol_: Text, decimals_: Nat8, totalSupply_: Nat){
    type TransactionId = Types.TransactionId;
    type TokenHolder = Types.TokenHolder;
    type MetaData = Types.MetaData;
    type CallData = Types.CallData;
    type TxRecord = Types.TxRecord;
    type ApproveResult = Types.ApproveResult;
    type TransferResult = Types.TransferResult;
    type BurnResult = Types.BurnResult;

    type StorageActor = actor {
        graphql_query_custom: query (Text, Text) -> async (Text);
        graphql_mutation: (Text, Text) -> async (Text);
    };

    type TokenReceiverActor = actor {
      supportedInterface : query(methodSig :Text) -> async Bool;
      on_token_received: shared(transferFrom: TokenHolder, value: Nat) -> async Bool;
    };

    private stable var _owner : Principal = msg.caller;
    private stable var _name : Text = name_;
    private stable var _symbol : Text = symbol_;
    private stable var _decimals : Nat8 = decimals_;
    private stable var _totalSupply : Nat = totalSupply_;
    private stable var _fee : Types.Fee = { lowest = 0 ; rate = 0 ; } ;
    private stable var _txIdCursor : Nat = 0 ;

    private stable var _logo: [Nat8] = [];
    private stable var _feeTo : TokenHolder = #Principal(msg.caller) ;

    private stable var _extendDataEntries : [(Text, Text)] = [];
    private stable var _balanceEntries : [(TokenHolder, Nat)] = [];
    private stable var _allowanceEntries : [(TokenHolder, [(TokenHolder, Nat)])] = [];

    private var _balances = HashMap.HashMap<TokenHolder, Nat>(1, Types.TokenHolder.equal, Types.TokenHolder.hash);
    private var _allowances = HashMap.HashMap<TokenHolder, HashMap.HashMap<Types.TokenHolder, Nat>>(1, Types.TokenHolder.equal, Types.TokenHolder.hash);
    private var _extendDatas = HashMap.HashMap<Text, Text>(1, Text.equal, Text.hash);

    private stable var _storageCanister : ?StorageActor = null;
    
    private let MSG_ONLY_OWNER = "DFT: caller is not the owner";
    private let MSG_INVALID_SPENDER = "DFT: invalid spender";
    private let MSG_INVALID_FROM = "DFT: invalid format [from]";
    private let MSG_INVALID_TO = "DFT: invalid format [to]";
    private let MSG_INVALID_FEE_TO = "DFT: invalid format [feeTo]";
    private let MSG_FAILED_TO_CHARGE_FEE = "DFT: Failed to charge fee - insufficient balance";
    private let MSG_ALLOWANCE_EXCEEDS = "DFT: transfer amount exceeds allowance";
    private let MSG_BALANCE_EXCEEDS = "DFT: transfer amount exceeds balance";
    private let MSG_BURN_VALUE_TOO_SMALL = "DFT: burning value is too small";
    private let MSG_BURN_VALUE_EXCEEDS = "DFT: burning value exceeds balance";
    private let MSG_NOTIFICATION_FAILED = "DFT: notification failed";
   
    private let DECIMALS_FEE_RATE: Nat = 8;
    private let TX_TYPES_APPROVE: Text = "approve";
    private let TX_TYPES_TRANSFER: Text = "transfer";
    private let TX_TYPES_BURN: Text = "burn";
    // private let TX_TYPES_MINT: &str = "mint";


    _balances.put(#Principal(_owner), totalSupply_);

    public query func name() : async Text { name_ };
    public query func symbol() : async Text { _symbol };

    public query func decimals() : async Nat8 { _decimals };

    public query func totalSupply() : async Nat { _totalSupply };

    public query func fee() : async Types.Fee { _fee };

    public query func meta() : async MetaData {
      return {
        name = name_;
        symbol = symbol_;
        decimals = _decimals;
        total_supply = _totalSupply;
        fee = _fee;
      };
    };

    public query func extend() : async [Types.KeyValuePair] {
      return Types.KeyValuePair.mapToArray(_extendDatas);
    };

    public shared(msg) func updateExtend( extendDatas: [Types.KeyValuePair]) : async Bool {
      if ( _owner != msg.caller ) { throw Error.reject(MSG_ONLY_OWNER); };
      
      for (v in extendDatas.vals()) {
        if (Types.ExtendData.isValidKey(v.k)){
           _extendDatas.put( v.k , v.v );
        }
        else throw Error.reject("Not support key " # v.k);
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
        let aid = AID.fromPrincipal(msg.caller, subAccount);
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

    public shared(msg) func burn (subAccount: ?AID.Subaccount, value: Nat) : async BurnResult {       
      var fromHolderParseResult = ?Types.TokenHolder.fromPrincipal(msg.caller);
      if ( Option.isSome(subAccount)) {
        let aid = AID.fromPrincipal(msg.caller, subAccount);
        fromHolderParseResult := ?Types.TokenHolder.fromAid(aid);
      };
      
      let fromHolder = Option.unwrap(fromHolderParseResult) ;

      return await _burn (fromHolder, value);
    };

    private func _burn(from: TokenHolder, value: Nat) : async BurnResult {
      let fee = _calcTransferFee(value); 
      if (value < fee) { return #Err(MSG_BURN_VALUE_TOO_SMALL); };
      let balance = _balanceOf(from);
      if (balance < value) { return #Err(MSG_BURN_VALUE_EXCEEDS); };
      let newBalance :Nat = balance - value;

      if ( newBalance > 0 ){ _balances.put(from, newBalance); }
      else { _balances.delete( from ); };

      _totalSupply -= value;

      ignore  _saveTxRecordToGraphql(#Burn{
         from = from;        
         value = value;
         timestamp = Time.now();
      });
      return #Ok(());
    };

    public query func supportedInterface(interfaceSig: Text) : async Bool {
       var did = _getDid();
       did := Text.replace(did, #text " ", "");
       let interfaceSigRep = Text.replace(interfaceSig, #text " ", "");
       Text.contains(did,#text interfaceSigRep)
    };

    public shared(msg) func setStorageCanisterID (storageCanisterId: ?Principal) : async Bool {       
      if ( _owner != msg.caller ) { throw Error.reject(MSG_ONLY_OWNER); };
      if ( storageCanisterId == null ) { _storageCanister := null; }
      else { _storageCanister := ?actor(Principal.toText(Option.unwrap(storageCanisterId))); };
      true
    };

    public shared(msg) func setFee (fee: Types.Fee) : async Bool {       
      if ( _owner != msg.caller ) { throw Error.reject(MSG_ONLY_OWNER); };
      _fee := fee;
      true
    };

    public shared(msg) func setFeeTo (feeTo: Text) : async Bool {       
      if ( _owner != msg.caller ) { throw Error.reject(MSG_ONLY_OWNER); };
      let feeToHolderParseResult = Types.TokenHolder.fromText(feeTo);
      if (Option.isNull( feeToHolderParseResult )) throw Error.reject(MSG_INVALID_FEE_TO);
      let feeToHolder = Option.unwrap(feeToHolderParseResult);
      true
    };

    private func _setFeeTo(feeTo: TokenHolder) : Bool {
      _feeTo := feeTo;
      true
    };

    public query func tokenGraphql() : async ?Principal {
      if( _storageCanister != null) return ?Principal.fromActor (Option.unwrap(_storageCanister));
       
      return null;
    };

    public query func cyclesBalance() : async Nat {
      return ExperimentalCycles.balance();
    };

    public func receiveCycles() : async () {
      let amount = ExperimentalCycles.available(); 
      let accepted = ExperimentalCycles.accept(amount);
      assert (accepted == amount);
    };


    public query func __export_did_tmp( ) : async Text {
      // copy from .dfx/local/canisters/dft_motoko/dft_motoko.did
      // get the $content from type Token = $content;
      _getDid()   
    };

    private func _getDid() : Text {
      "service { " # 
      "allowance: (text, text) -> (nat) query;" #
      "approve: (opt Subaccount, text, nat, opt CallData) -> (ApproveResult);" #
      "balanceOf: (text) -> (nat) query;" #
      "burn: (opt Subaccount, nat) -> (BurnResult);" #
      "cyclesBalance: () -> (nat) query;" #
      "decimals: () -> (nat8) query;" #
      "extend: () -> (vec record {text;text;}) query;" #
      "fee: () -> (Fee) query;" #
      "logo: () -> (vec nat8) query;" #
      "meta: () -> (MetaData) query;" #
      "name: () -> (text) query;" #
      "receiveCycles: () -> ();" #
      "setFee: (Fee) -> (bool);" #
      "setFeeTo: (text) -> (bool);" #
      "setStorageCanisterID: (opt principal) -> (bool);" #
      "supportedInterface: (text) -> (bool) query;" #
      "symbol: () -> (text) query;" #
      "tokenGraphql: () -> (opt principal) query;" #
      "totalSupply: () -> (nat) query;" #
      "transfer: (opt Subaccount, text, nat, opt CallData) -> (TransferResult);" #
      "transferFrom: (opt Subaccount, text, text, nat) -> (TransferResult);" #
      "updateExtend: (vec record {text;text;}) -> (bool);" #
      "updateLogo: (vec nat8) -> (bool); };"
    };

    // TODO:Can not perform call in a generic way,I am looking for a solution
    // https://github.com/dfinity/motoko/issues/2703 waiting motoko support 
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
      //TODO: impl save tx to graphql
      _txIdCursor += 1;
      if ( _storageCanister == null){
        return _txIdCursor;
      };
      var typeTxt : Text = "";
      var fromTxt : Text = "";
      var toTxt : Text = "";
      var valueTxt : Text = "";
      var feeTxt : Text = "";
      var timestampTxt : Text = "";
      switch (tx) {
        case (#Approve { owner; spender; value; fee; timestamp; }) {
          typeTxt := TX_TYPES_APPROVE ;
          fromTxt := Types.TokenHolder.toText(owner) ;
          toTxt := Types.TokenHolder.toText(spender) ;
          valueTxt := Nat.toText(value) ;
          feeTxt :=  Nat.toText(fee) ;
          timestampTxt :=  Int.toText(timestamp) ;
        } ;
        case (#Transfer { from; to; value; fee; timestamp; }) {
          typeTxt := TX_TYPES_TRANSFER ;
          fromTxt := Types.TokenHolder.toText(from) ;
          toTxt := Types.TokenHolder.toText(to) ;
          valueTxt := Nat.toText(value) ;
          feeTxt :=  Nat.toText(fee) ;
          timestampTxt :=  Int.toText(timestamp) ;
        } ;
        case (#Burn { from; value; timestamp; }) {
          typeTxt := TX_TYPES_BURN ;
          fromTxt := Types.TokenHolder.toText(from) ;
          toTxt := "" ;
          valueTxt := Nat.toText(value) ;
          feeTxt :=  "0" ;
          timestampTxt := Int.toText(timestamp) ;
        } ;
      };
      
      var muation = "mutation { createTx(input: { " #
        "txid: \"" # Nat.toText(_txIdCursor) #  "\"," #
        "txtype: \"" # typeTxt # "\",from: \"" # fromTxt #  "\"," #
        "to: \"" # toTxt # "\",value: \"" # valueTxt #  "\"," #
        "fee: \"" # feeTxt #  "\"," #
        "timestamp: \"" # timestampTxt # "\" }) { id } }";

      Debug.print("muation is: " # muation);

      ignore Option.unwrap(_storageCanister).graphql_mutation(muation, "{}");

      return _txIdCursor;
    };

    private func _onTokenReceived(from: TokenHolder, to: TokenHolder, value: Nat) : async Result.Result<Bool, Text> {
      let receiverCanisterId : ?Principal  = switch(to) {
        case (#Account accountID) null;
        case (#Principal principal) {
          if (PrincipalExt.isCanister(principal)) { ?principal ; }
          else null;
        };
      };

      if(receiverCanisterId == null) return #ok(true);
      let on_token_received_method_sig = "on_token_received:(TokenHolder,nat)->(bool)query";
      let receiverCanister : TokenReceiverActor = actor(Types.TokenHolder.toText(to));

      let isSupportHook : Bool = await receiverCanister.supportedInterface(on_token_received_method_sig);
      if (isSupportHook != true) return #ok(true);

      let notifyResult : Bool = await receiverCanister.on_token_received (from , value);

      if (notifyResult != true) return #err(MSG_NOTIFICATION_FAILED);
      
      return #ok(true); 
    };

    // do something becore sending if you want
    private func _onTokenSending(from: TokenHolder, to: TokenHolder, value: Nat) : Result.Result<(), Text> 
    {
      #ok(());
    };

    system func preupgrade() {
        _balanceEntries := Iter.toArray(_balances.entries());
        let allowanceSize : Nat = _allowances.size();
        var tmpAllowanceArr : [var (TokenHolder, [(TokenHolder, Nat)])] = Array.init<(TokenHolder, [(TokenHolder, Nat)])>(allowanceSize, (_feeTo, []));
        
        var index : Nat = 0;
        for ((k, v) in _allowances.entries()) {
            tmpAllowanceArr[index] := (k, Iter.toArray(v.entries()));
            index += 1;
        };
        _allowanceEntries := Array.freeze(tmpAllowanceArr);
    };

    system func postupgrade() {
        _balances := HashMap.fromIter<TokenHolder, Nat>(_balanceEntries.vals(), 1, Types.TokenHolder.equal, Types.TokenHolder.hash);
        _balanceEntries := [];
        for ((k, v) in _allowanceEntries.vals()) {
            let allowanceTemp = HashMap.fromIter<TokenHolder, Nat>(v.vals(), 1, Types.TokenHolder.equal, Types.TokenHolder.hash);
            _allowances.put(k, allowanceTemp);
        };
        _allowanceEntries := [];
    };
};
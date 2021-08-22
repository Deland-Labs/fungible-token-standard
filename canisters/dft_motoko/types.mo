import Text "mo:base/Text";
import Hash "mo:base/Hash";
import Array "mo:base/Array";
import Debug "mo:base/Debug";
import HashMap "mo:base/HashMap";
import Option "mo:base/Option";
import Principal "mo:base/Principal";
import Nat8 "mo:base/Nat8";
import Nat "mo:base/Nat";
import AID "./utils/AccountIdentifier";
import PrincipalExt "./utils/PrincipalExt";
import CRC32 "./utils/CRC32";
import Time "mo:base/Time";
import Prim "mo:⛔";

module {
  public type Subaccount = [Nat8];
  public type TransactionId = Nat;
  // Rate decimals = 8
  // transferFee = cmp::max(lowest;amount * rate / 10^8)
  public type Fee ={
    lowest: Nat;
    rate: Nat;
  };

  public type MetaData = {
    name: Text ;
    symbol: Text ;
    decimals: Nat8 ;
    total_supply: Nat ;
    fee: Fee ;
  }; 

  public type TokenHolder = {
    #Account: AID.AccountIdentifier ;
    #Principal: Principal ;
  };

  public module TokenHolder {
    public func fromText(text : Text) : ?TokenHolder {
      let parseResult = PrincipalExt.fromText(text);

      if (Option.isSome(parseResult)){
          return  ?#Principal(Option.unwrap(parseResult))
      };
      
      let aidParseResult = AID.fromText( text ) ;

      if (Option.isSome(aidParseResult)){
          return ?#Account(Option.unwrap(aidParseResult))
      };
      return null;
    };

    public func fromPrincipal(p : Principal) : TokenHolder {       
      return #Principal(p);     
    };

     public func fromAid(aid : AID.AccountIdentifier) : TokenHolder {       
      return #Account(aid);     
    };
    
    public func toPrincipal(holder : TokenHolder) : ?Principal {
      switch(holder) {
        case (#Account accountID) null;
        case (#Principal principal) ?principal;
      }; 
    };

    public func toText(holder : TokenHolder) : Text {
      switch(holder) {
        case (#Account accountID) AID.toText(accountID);
        case (#Principal principal) Principal.toText(principal);
      }; 
    };

    public func equal(x : TokenHolder, y : TokenHolder) : Bool {
      let _x = switch(x) {
        case (#Account accountID) AID.hash(accountID);
        case (#Principal principal) Principal.hash(principal);
      };
      let _y = switch(y) {
        case (#Account accountID) AID.hash(accountID);
        case (#Principal principal) Principal.hash(principal);
      };
      return _x==_y;
    };

    public func hash(x : TokenHolder) : Hash.Hash {
      let _x = switch(x) {
        case (#Account accountID) return AID.hash(accountID);
        case (#Principal principal) return Principal.hash(principal);
      };      
    };
  };

  public module ExtendData{
    let OFFICIAL_SITE: Text = "OFFICIAL_SITE";
    let MEDIUM: Text = "MEDIUM";
    let OFFICIAL_EMAIL: Text = "OFFICIAL_EMAIL";
    let DESCRIPTION: Text = "DESCRIPTION";
    let BLOG: Text = "BLOG";
    let REDDIT: Text = "REDDIT";
    let SLACK: Text = "SLACK";
    let FACEBOOK: Text = "FACEBOOK";
    let TWITTER: Text = "TWITTER";
    let GITHUB: Text = "GITHUB";
    let TEGEGRAM: Text = "TEGEGRAM";
    let WECHAT: Text = "WECHAT";
    let LINKEDIN: Text = "LINKEDIN";
    let DISCORD: Text = "DISCORD";
    let WHITE_PAPER: Text = "WHITE_PAPER";

    let DSCVR: Text = "DSCVR";
    let OPENCHAT: Text = "OPENCHAT";
    let DISTRIKT: Text = "DISTRIKT";
    let WEACT: Text = "WEACT";

    let  EXTEND_KEYS:[Text]=[
      DSCVR,
      OPENCHAT,
      DISTRIKT,
      WEACT,
      OFFICIAL_SITE,
      MEDIUM,
      OFFICIAL_EMAIL,
      DESCRIPTION,
      BLOG,
      REDDIT,
      SLACK,
      FACEBOOK,
      TWITTER,
      GITHUB,
      TEGEGRAM,
      WECHAT,
      LINKEDIN,
      DISCORD,
      WHITE_PAPER,
    ];

    public func isValidKey(key : Text) : Bool {
      let existKey =  Array.filter(EXTEND_KEYS,func (x : Text) : Bool { key == x });
      if (existKey.size() > 0) { return true; };
      
      return false;
    };
  };

  public type TransferFrom = TokenHolder;
  public type TokenReceiver = TokenHolder;

  public type KeyValuePair = {
      k : Text;
      v : Text;
  };

  public module KeyValuePair{
    public func mapToArray(map : HashMap.HashMap<Text, Text>) : [KeyValuePair] {
      var array:[var KeyValuePair] =  Array.init<KeyValuePair>(map.size(), {k = ""; v = "";});
      var index : Nat = 0;
      for( (k , v) in map.entries()){
        array[index] := { k = k; v = v; };
        index += 1;
      };
      return Array.freeze(array);
    };
  };

  public type CallData = {
      method : Text;
      args : [Nat8];
  };

  public type TransferResult = {
      //transfer succeed; but call failed & notify failed
      #Ok : { txid : TransactionId; error : ?[Text]};
      #Err : Text;
  };

  public type BurnResult = {
      #Ok : ();
      #Err : Text;
  };

  public type ApproveResult = {
      #Ok : ?Text;
      #Err : Text;
  };

  public type TxRecord = {
      #Approve:{
          owner : TokenHolder; 
          spender: TokenReceiver;
          value: Nat; 
          fee: Nat;
          timestamp: Time.Time;
    };
    #Transfer:{
        from: TokenHolder;
        to: TokenReceiver;
        value: Nat; 
        fee: Nat;
        timestamp: Time.Time;
    };
    #Burn:{
        from: TokenHolder; 
        value: Nat;
        timestamp: Time.Time;
    }
  };
}
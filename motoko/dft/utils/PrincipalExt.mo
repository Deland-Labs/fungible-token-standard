import Text "mo:base/Text";
import Base32 "./Base32";
import CRC32 "./CRC32";
import Principal "mo:base/Principal";
import List "mo:base/List";
import Array "mo:base/Array";
import Blob "mo:base/Blob";
import Prim "mo:⛔";

module {
    let CRC_LENGTH_IN_BYTES: Nat = 4;
    let CANISTER_ID_HASH_LEN_IN_BYTES: Nat = 10;
    let HASH_LEN_IN_BYTES: Nat = 28;
    let MAX_LENGTH_IN_BYTES: Nat = 29; //HASH_LEN_IN_BYTES + 1; // 29
    let TYPE_SELF_AUTH: Nat8 = 0x02;

    public func fromText(text : Text) : ?Principal {
      var _text = Text.map(text , Prim.charToLower);
      _text := Text.replace(_text , #text "-" , "");
      let decodeResult = Base32.decode(#RFC4648({ padding=false; }),_text);
      let bytes:[Nat8] = switch (decodeResult)
      {
        case null [];
        case (?b) b;
      };
      
      let bytesSize = bytes.size();

      if ( bytes.size() < CRC_LENGTH_IN_BYTES ) { return null; }
      else if ( bytes.size() > MAX_LENGTH_IN_BYTES + CRC_LENGTH_IN_BYTES) { return null; }
      else if ( text == "aaaaa-aa") { return ?Principal.fromText(text); }
      else {
        let body = Array.init<Nat8>(bytesSize - 4, 0) ;
      
        for (k in bytes.keys()) {     
           if ( k > 3 ) { 
             body[ k - 4 ] := bytes [ k ];
           }
        };

        let crcResult : [Nat8] = CRC32.crc32(Array.freeze(body));

        for (c in crcResult.keys()){
          if ( bytes[c] != crcResult[c]) {
            return null;
          }
        };

        return ?Principal.fromText(text);
      };
    };

    public func isCanister(id: Principal) : Bool {
      let bytes = Blob.toArray(Principal.toBlob(id));
      bytes.size() == CANISTER_ID_HASH_LEN_IN_BYTES
    };

    public func isUserPrincipal(id: Principal) : Bool {
      let bytes = Blob.toArray(Principal.toBlob(id));
      if (bytes.size() != HASH_LEN_IN_BYTES + 1) {
        return false;
      };
      if (bytes[bytes.size() - 1] != TYPE_SELF_AUTH) {
        return false;
      };
      true
    };

};
import Nat "mo:base/Nat";
import Nat8 "mo:base/Nat8";
import Nat32 "mo:base/Nat32";
import Char "mo:base/Char";
import Option "mo:base/Option";
import Principal "mo:base/Principal";
import Blob "mo:base/Blob";
import Hash "mo:base/Hash";
import HashMap "mo:base/HashMap";
import Iter "mo:base/Iter";
import Array "mo:base/Array";
import Text "mo:base/Text";
import SHA224 "./SHA224";
import CRC32 "./CRC32";

module {
  public type AccountIdentifier = {
    hash: [Nat8];
  };
  public type Subaccount = [Nat8];
  
  private let symbols = ['0', '1', '2', '3', '4', '5', '6', '7','8', '9', 'a', 'b', 'c', 'd', 'e', 'f',];
  private let base : Nat8 = 0x10;
  private let ads : [Nat8] = [10, 97, 99, 99, 111, 117, 110, 116, 45, 105, 100]; //b"\x0Aaccount-id"
  public let SUBACCOUNT_ZERO : [Nat8] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

  
  public func fromText(t : Text) : ?AccountIdentifier {
    var map = HashMap.HashMap<Nat, Nat8>(1, Nat.equal, Hash.hash);
    // '0': 48 -> 0; '9': 57 -> 9
    for (num in Iter.range(48, 57)) {
      map.put(num, Nat8.fromNat(num-48));
    };
    // 'a': 97 -> 10; 'f': 102 -> 15
    for (lowcase in Iter.range(97, 102)) {
      map.put(lowcase, Nat8.fromNat(lowcase-97+10));
    };
    // 'A': 65 -> 10; 'F': 70 -> 15
    for (uppercase in Iter.range(65, 70)) {
      map.put(uppercase, Nat8.fromNat(uppercase-65+10));
    };
    let p = Iter.toArray(Iter.map(Text.toIter(t), func (x: Char) : Nat { Nat32.toNat(Char.toNat32(x)) }));
    var res : [var Nat8] = [var];
    var crc : [var Nat8] = [var];
    for (i in Iter.range(0, 3)) {            
      let a = Option.unwrap(map.get(p[i*2]));
      let b = Option.unwrap(map.get(p[i*2 + 1]));
      let c = 16*a + b;
      crc := Array.thaw(Array.append(Array.freeze(crc), Array.make(c)));
    };        
    for (i in Iter.range(4, 31)) {            
      let a = Option.unwrap(map.get(p[i*2]));
      let b = Option.unwrap(map.get(p[i*2 + 1]));
      let c = 16*a + b;
      res := Array.thaw(Array.append(Array.freeze(res), Array.make(c)));
    };
    let result = Array.freeze(res);
    if(Array.freeze(crc) != CRC32.crc32(result)) return null;
    return ?({hash = result;} : AccountIdentifier);
  };

  public func fromPrincipal(p : Principal, sa : ?Subaccount) : AccountIdentifier {
    return fromBlob(Principal.toBlob(p), sa);
  };

  public func fromBlob(b : Blob, sa : ?Subaccount) : AccountIdentifier {
    return fromBytes(Blob.toArray(b), sa);
  };

  public func fromBytes(data : [Nat8], sa : ?Subaccount) : AccountIdentifier {
    var _sa : [Nat8] = SUBACCOUNT_ZERO;
    if (Option.isSome(sa)) {
      _sa := Option.unwrap(sa);
    };
    var hash : [Nat8] = SHA224.sha224(Array.append(Array.append(ads, data), _sa));
    var crc : [Nat8] = CRC32.crc32(hash);
    return {hash = Array.append(crc, hash);} : AccountIdentifier;
  };

  public func toText(p : AccountIdentifier) : Text {
    let crc = CRC32.crc32(p.hash);
    let acountIdBytes = Array.append<Nat8>(crc, p.hash);

    return encode(acountIdBytes);
  }; 
  
  /// Return the [motoko-base's Hash.Hash](https://github.com/dfinity/motoko-base/blob/master/src/Hash.mo#L9) of `AccountIdentifier`.  
  /// To be used in HashMap.
  public func hash(a: AccountIdentifier) : Hash.Hash {
    var array : [Hash.Hash] = [];
    var temp : Hash.Hash = 0;
    for (i in a.hash.vals()) {
      temp := Hash.hash(Nat8.toNat(i));
      array := Array.append<Hash.Hash>(array, Array.make<Hash.Hash>(temp));
    };

    return Hash.hashNat8(array);
  };

  /// Test if two account identifier are equal.
  public func equal(a: AccountIdentifier, b: AccountIdentifier) : Bool {
    Array.equal<Nat8>(a.hash, b.hash, Nat8.equal)
  };

  /// Convert bytes array to hex string.       
  /// E.g `[255,255]` to "ffff"
  public func encode(array : [Nat8]) : Text {
        Array.foldLeft<Nat8, Text>(array, "", func (accum, u8) {
            accum # nat8ToText(u8);
        });
  };

  /// Convert a byte to hex string.
  /// E.g `255` to "ff"
  func nat8ToText(u8: Nat8) : Text {
    let c1 = symbols[Nat8.toNat((u8/base))];
    let c2 = symbols[Nat8.toNat((u8%base))];
    return Char.toText(c1) # Char.toText(c2);
  };
};
import type { Principal } from '@dfinity/principal';
export type TokenHolder = { 'None' : null } |
  { 'Account' : string } |
  { 'Principal' : Principal };
export interface _SERVICE {
  'onTokenReceived' : (arg_0: TokenHolder, arg_1: bigint) => Promise<boolean>,
}

import type { Principal } from '@dfinity/principal';
export interface _SERVICE {
  'notificationCount' : () => Promise<bigint>,
  'onTokenReceived' : (arg_0: string, arg_1: bigint) => Promise<undefined>,
}

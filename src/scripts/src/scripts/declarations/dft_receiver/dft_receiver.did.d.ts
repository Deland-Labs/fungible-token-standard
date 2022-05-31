import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface _SERVICE {
  'notificationCount' : ActorMethod<[], bigint>,
  'onTokenReceived' : ActorMethod<[bigint, string, bigint], undefined>,
}

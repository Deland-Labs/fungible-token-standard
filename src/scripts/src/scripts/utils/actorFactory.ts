import { Actor, getDefaultAgent, HttpAgent, Identity } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { identity, canister } from "@deland-labs/ic-dev-kit";
import logger from "node-color-log";
// import dfxConfig from "../../dfx.json";
export const IC_HOST = "https://ic0.app";

const isLocalEnv = true;

/* function getHost() {
  // Setting host to undefined will default to the window location
  return isLocalEnv ? dfxConfig.networks.local.bind : IC_HOST;
} */

// export const host = getHost();
export const host = IC_HOST;

class ActorFactory {
    private static _instance: ActorFactory = new ActorFactory();
    private static _agent: any;
    // actor cache, cache by canisterDid, canisterId and identity
    private _actorCache: { [canisterDid: string]: { [canisterId: string]: { [identity: string]: any } } } = {};

    public static getInstance() {
        return this._instance;
    }

    _isAuthenticated: boolean = false;

    createActorByName<T>(canisterDid: any, canisterName: string, identity_info: identity.IdentityInfo): T {
        let canister_id = canister.get_id(canisterName);
        return this.createActor(canisterDid, canister_id, identity_info.identity);
    }

    createActor<T>(canisterDid: any, canisterId: string | Principal, identity?: Identity) {
        let canister_id = canisterId.toString();
        let identity_str = identity ? identity.toString() : "default";
        // find actor from cache
        if (!(this._actorCache[canisterDid] && this._actorCache[canisterDid][canister_id] && this._actorCache[canisterDid][canister_id][identity_str])) {
            logger.info("Creating actor for canisterId: " + canister_id + " identity: " + identity_str);
            const agent = getDefaultAgent();
            const actor = Actor.createActor<T>(canisterDid, {
                agent,
                canisterId,
            });
            // The root key only has to be fetched for local development environments
            if (isLocalEnv) {
                agent.fetchRootKey().catch(console.error);
            }
            // cache actor
            if (!this._actorCache[canisterDid]) {
                this._actorCache[canisterDid] = {};
            }
            if (!this._actorCache[canisterDid][canister_id]) {
                this._actorCache[canisterDid][canister_id] = {};
            }
            this._actorCache[canisterDid][canister_id][identity_str] = actor;
        }
        return this._actorCache[canisterDid][canister_id][identity_str];
    }

    /*
     * Once a user has authenticated and has an identity pass this identity
     * to create a new actor with it, so they pass their Principal to the backend.
     */
    async authenticateWithIdentity(identity: Identity) {
        ActorFactory._agent = new HttpAgent({
            host,
            identity: identity
        });
        this._isAuthenticated = true;
    }

    /*
   * Once a user has authenticated and has an identity pass this identity
   * to create a new actor with it, so they pass their Principal to the backend.
   */
    async authenticateWithAgent(agent: HttpAgent) {
        ActorFactory._agent = agent;
        this._isAuthenticated = true;
    }

    /*
     * If a user unauthenticates, recreate the actor without an identity.
     */
    unauthenticateActor() {
        ActorFactory._agent = undefined;
        this._isAuthenticated = false;
    }
}

export const actorFactory = ActorFactory.getInstance();
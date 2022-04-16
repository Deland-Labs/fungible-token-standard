import {exec} from "shelljs";
import {Identity} from "@dfinity/agent";
import fs from "fs";
import {Secp256k1KeyIdentity} from "@dfinity/identity";
import sha256 from "sha256";
import {principalToAccountIDInBytes, toHexString} from "./convert";
import {Principal} from "@dfinity/principal";
import {get_id} from "~/utils/canister";
import logger from "node-color-log";

export function load(name: string): Identity {
    new_dfx_identity(name);
    // get current home directory
    const home = process.env.HOME;
    let pem_path = `${home}/.config/dfx/identity/${name}/identity.pem`;
    const rawKey = fs
        .readFileSync(pem_path)
        .toString()
        .replace("-----BEGIN EC PRIVATE KEY-----", "")
        .replace("-----END EC PRIVATE KEY-----", "")
        .trim();

    // @ts-ignore
    const rawBuffer = Uint8Array.from(rawKey).buffer;

    const privKey = Uint8Array.from(sha256(rawBuffer, {asBytes: true}));

    // Initialize an identity from the secret key
    return Secp256k1KeyIdentity.fromSecretKey(
        Uint8Array.from(privKey).buffer
    );
}

export const new_dfx_identity = (name: string) => {
    exec(`dfx identity new ${name}`, {silent: true});
}

export const useDfxIdentity = (name: string) => {
    exec(`dfx identity use ${name}`, {silent: true});
}

export interface agentOptions {
    host: string;
    identity: Identity;
}

export interface IdentityInfo {
    identity: Identity;
    principalText: string;
    agentOptions: agentOptions;
}

const DEFAULT_HOST = "http://127.0.0.1:8000";
export const IDENTITIES = ["dft_main", "dft_miner", "dft_user1", "dft_user2", "dft_user3", "dft_user4", "dft_user5",
    "dft_user6", "dft_user7", "dft_user8", "dft_user9", "dft_user10", "dft_user11", "dft_user12", "dft_user13",
    "dft_receiver", "dft_fee_charger"];
export const DEFAULT_IDENTITY = IDENTITIES[0];
export const createIdentities = () => {
    IDENTITIES.forEach(new_dfx_identity);
}

class IdentityFactory {
    private _identities: Map<string, IdentityInfo>;

    constructor() {
        useDfxIdentity(DEFAULT_IDENTITY);
        this._identities = new Map<string, IdentityInfo>();
    }

    private loadIdentityInfo = (name: string) => {
        const identity = load(name);
        const principal = identity.getPrincipal();
        const identityInfo: IdentityInfo = {
            identity: identity,
            principalText: principal.toText(),
            agentOptions: {
                host: DEFAULT_HOST,
                identity: identity,
            }
        };
        this._identities.set(name, identityInfo);
    }

    getDefaultHost = () => {
        return DEFAULT_HOST;
    }

    loadAllIdentities() {
        IDENTITIES.forEach(this.loadIdentityInfo);
    }

    getIdentity = (name?: string): IdentityInfo | undefined => {
        return this._identities.get(name || DEFAULT_IDENTITY);
    }

    getPrincipal = (name?: string): Principal | undefined => {
        const identityInfo = this.getIdentity(name || DEFAULT_IDENTITY);
        if (identityInfo) {
            return identityInfo.identity.getPrincipal();
        }
        return undefined;
    }

    getAccountIdHex = (name?: string, index?: number): string | undefined => {
        const identityInfo = this.getIdentity(name || DEFAULT_IDENTITY);
        if (identityInfo) {
            const principal = identityInfo.identity.getPrincipal();
            const accountIdUint8 = principalToAccountIDInBytes(principal, this.getSubAccount(index ?? 0));
            return toHexString(accountIdUint8);
        }
        return undefined;
    }

    getAccountIdBytes = (name?: string, index?: number): Array<number> | undefined => {
        const identityInfo = this.getIdentity(name || DEFAULT_IDENTITY);
        if (identityInfo) {
            const principal = identityInfo.identity.getPrincipal();
            const accountIdUint8 = principalToAccountIDInBytes(principal);
            return Array.from(accountIdUint8);
        }
        return undefined;
    }

    getSubAccount = (index: number) => {
        let subAccount = new Uint8Array(32).fill(0);
        subAccount[0] = index;
        return subAccount;
    }
}

export const identityFactory = new IdentityFactory();
identityFactory.loadAllIdentities();
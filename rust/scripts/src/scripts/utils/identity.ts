import {exec} from "shelljs";
import {Identity} from "@dfinity/agent";
import fs from "fs";
import {Secp256k1KeyIdentity} from "@dfinity/identity";
import sha256 from "sha256";
import {principalToAccountIDInBytes, toHexString} from "./convert";
import {Principal} from "@dfinity/principal";
import {get_id} from "~/utils/canister";

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

export const use_dfx_identity = (name: string) => {
    exec(`dfx identity use ${name}`, {silent: true});
}

export interface IdentityDfxInfo {
    principal_text: string;
    account_id: string;
}

export interface agentOptions {
    host: string;
    identity: Identity;
}

export interface IdentityInfo {
    identity: Identity;
    principal_text: string;
    agentOptions: agentOptions;
    account_id_hex: string;
    account_id_bytes: Array<number>;
    subaccount1_id_bytes: Array<number>;
    subaccount2_id_bytes: Array<number>;
    subaccount3_id_bytes: Array<number>;
}

export interface IdentityCollection {
    main: IdentityInfo;
    miner: IdentityInfo,
    user1: IdentityInfo,
    user2: IdentityInfo,
    user3: IdentityInfo,
    subaccount1: Array<number>,
    subaccount2: Array<number>,
    subaccount3: Array<number>,
    // subaccount for dft ledger to receive quota order payment
    registrar_quota_order_receiver_subaccount: Array<number>,
    // subaccount for dft ledger to refund quota order payment
    registrar_quota_order_refund_subaccount: Array<number>,

    get_identity_info(name: string): IdentityInfo;

    get_principal(name: string): Principal
}

export const create_identities = () => {
    new_dfx_identity("dft_main");
    new_dfx_identity("dft_miner");
    new_dfx_identity("dft_user1");
    new_dfx_identity("dft_user2");
    new_dfx_identity("dft_user3");
}

export const identities = ((): IdentityCollection => {
    const get_subaccount = (index: number) => {
        let subAccount = new Uint8Array(32).fill(0);
        subAccount[0] = index;
        return subAccount;
    }

    const create_identities = (name: string): IdentityInfo => {

        const identity = load(name);
        const principal = identity.getPrincipal();
        const account_id_uint8 = principalToAccountIDInBytes(principal);
        const account_id_bytes = Array.from(account_id_uint8);
        return {
            identity: identity,
            principal_text: principal.toText(),
            agentOptions: {
                host: "http://127.0.0.1:8000",
                identity: identity,
            },
            account_id_hex: toHexString(account_id_uint8),
            account_id_bytes: account_id_bytes,
            subaccount1_id_bytes: Array.from(principalToAccountIDInBytes(principal, (get_subaccount(1)))),
            subaccount2_id_bytes: Array.from(principalToAccountIDInBytes(principal, (get_subaccount(2)))),
            subaccount3_id_bytes: Array.from(principalToAccountIDInBytes(principal, (get_subaccount(3)))),
        };
    }

    const default_identities = create_identities("dft_main");
    const miner_identities = create_identities("dft_miner");
    const user1_identities = create_identities("dft_user1");
    const user2_identities = create_identities("dft_user2");
    const user3_identities = create_identities("dft_user3");

    // reset to default in the end
    use_dfx_identity("dft_main");

    return {
        main: default_identities,
        miner: miner_identities,
        user1: user1_identities,
        user2: user2_identities,
        user3: user3_identities,
        subaccount1: Array.from(get_subaccount(1)),
        subaccount2: Array.from(get_subaccount(2)),
        subaccount3: Array.from(get_subaccount(3)),
        registrar_quota_order_receiver_subaccount: Array.from(get_subaccount(0x11)),
        registrar_quota_order_refund_subaccount: Array.from(get_subaccount(0x12)),
        get_identity_info(name: string): IdentityInfo {
            return this[name]
        },
        get_principal(name: string): Principal {
            let identityInfo = identities.get_identity_info(name);
            let user_principal: Principal;
            if (identityInfo == null) {
                user_principal = Principal.fromText(name);
            } else {
                user_principal = identityInfo.identity.getPrincipal();
            }
            return user_principal;
        },
    }
})();

export const identities_to_json = (identities: IdentityCollection): string => {
    // serialize identities as json
    // if property is Array<number>, convert to hex string
    // ignore agentOptions
    return JSON.stringify(identities, (key, value) => {
        if (key === "agentOptions") {
            return undefined;
        }
        if (key === "identity") {
            return undefined;
        }
        if (Array.isArray(value)) {
            return toHexString(Uint8Array.from(value));
        }
        return value;
    }, 2);
}
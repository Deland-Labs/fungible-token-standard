import {exec} from "shelljs";
import {Actor, CanisterInstallMode, HttpAgent} from "@dfinity/agent";
import {DfxJsonCanister, get_dfx_json, get_wasm_path} from "~/utils/dfxJson";
import fs from "fs";
import {identityFactory} from "~/utils/identity";
import logger from "node-color-log";

export const create = (name: string) => {
    const result = exec(`dfx canister create ${name}`);
    if (result.code !== 0) {
        throw new Error(result.stderr);
    }
}

export const uninstall_code = async (name: string) => {
    const result = exec(`dfx canister uninstall-code ${name}`);
    if (result.code !== 0) {
        throw new Error(result.stderr);
    }
    let max_retries = 30
    for (let i = 0; i < max_retries; i++) {
        const info_result = exec(`dfx canister info ${name}`);
        if (info_result.code !== 0) {
            throw new Error(info_result.stderr);
        }
        const info = info_result.stdout;
        if (info.includes("Module hash: None")) {
            logger.debug(`${name} uninstallation complete`);
            return;
        } else {
            logger.debug(`${name} uninstallation in progress... ${i}/${max_retries}`);
            await new Promise(resolve => setTimeout(resolve, 1000));
        }
    }
}

export const createAll = async () => {
    const result = exec(`dfx canister create --all`);
    if (result.code !== 0) {
        throw new Error(result.stderr);
    }
}

export const addMainAsController = async () => {
    // add main identity as controller of all canisters
    const update_result = exec(`dfx canister update-settings --all --add-controller ${identityFactory.getPrincipal()}`);
    if (update_result.code !== 0) {
        throw new Error(update_result.stderr);
    }
}

export const build = (name: string, features?: string) => {
    let dfx_json = get_dfx_json();
    let canister: DfxJsonCanister = dfx_json.canisters.get(name) as DfxJsonCanister;
    if (!canister) {
        throw new Error(`Canister ${name} not found in dfx.json`);
    }

    if (canister["type"] === "custom" && !canister.build) {
        logger.debug(`Canister ${name} is a custom canister without build scripts, skipping build`);
        return;
    }

    if (features) {
        logger.debug(`Building canister ${name} with features ${features}`);
        if (canister.build) {
            // run each line of build by replace dev_canister with features
            canister.build.forEach(line => {
                line = line.replace("dev_canister", features);
                console.info(`Running build command: ${line}`);
                const result = exec(line);
                if (result.code !== 0) {
                    throw new Error(result.stderr);
                }
            });
        } else {
            throw new Error(`Canister ${name} does not have a build section, so it cannot be built with features`);
        }
    } else {
        logger.debug(`Building canister ${name}`);
        const result = exec(`dfx build ${name}`);
        if (result.code !== 0) {
            throw new Error(result.stderr);
        }
    }
}

export const build_all = () => {
    const result = exec(`dfx build`);
    if (result.code !== 0) {
        throw new Error(result.stderr);
    }
    return result;
}

export const reinstall = (name: string, args?: string) => {
    console.info(`Reinstalling ${name}`);
    let result;
    if (args) {
        const command = `echo yes | dfx canister install --mode reinstall ${name} --argument ${args}`;
        console.info(`Reinstalling command ${command}`);
        result = exec(command, {silent: true});

    } else {
        result = exec(`echo yes | dfx canister install --mode reinstall ${name}`, {silent: true});
    }
    if (result.code !== 0) {
        throw new Error(result.stderr);
    }
    console.info(`${name} reinstalled`);
}

export const reinstall_code = async (name: string, args?: ArrayBuffer) => {
    console.info(`Reinstalling ${name}`);
    let dfxJson = get_dfx_json();
    let canister: DfxJsonCanister = dfxJson.canisters.get(name) as DfxJsonCanister;
    let wasmPath = get_wasm_path(canister);
    let buffer = fs.readFileSync(wasmPath);
    let canister_id = get_id(name);
    let agent = new HttpAgent(identityFactory.getIdentity()!.agentOptions);
    await agent.fetchRootKey();
    await Actor.install({
        module: buffer,
        mode: CanisterInstallMode.Reinstall,
        arg: args
    }, {
        canisterId: canister_id,
        agent: agent,
    })
    console.info(`${name} reinstalled`);
}

export const call = (name: string, method: string, args?: string) => {
    let result;
    if (args) {
        result = exec(`dfx canister call ${name} ${method} ${args}`);

    } else {
        result = exec(`dfx canister call ${name} ${method}`);
    }
    if (result.code !== 0) {
        throw new Error(result.stderr);
    }
}


export const get_id = (name: string) => {
    return exec(`dfx canister id ${name}`, {silent: true}).stdout.trim();
}

export interface ReInstallOptions {
    build?: boolean;
    init?: boolean;
}

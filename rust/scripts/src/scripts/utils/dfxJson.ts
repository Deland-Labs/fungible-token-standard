import fs from "fs";

export interface DfxJsonCanister {
    "type": string,
    package: string,
    candid: string,
    dependencies?: string[],
    wasm?: string,
    build?: string[],
    pack_config?: DfxPackageCanister
}

export const get_wasm_path = (canister: DfxJsonCanister): string => {
    if (canister?.wasm) {
        return canister.wasm
    }
    return `target/wasm32-unknown-unknown/release/${canister.package}.wasm`
}

export interface DfxJson {
    canisters: Map<string, DfxJsonCanister>,
}

export class DfxJsonFile implements DfxJson {
    canisters: Map<string, DfxJsonCanister> = new Map();
    private readonly path: string;

    constructor(path: string = "./dfx.json") {
        this.path = path ?? "./dfx.json";
        this.load();
    }

    private load() {
        if (fs.existsSync(this.path)) {
            const json = fs.readFileSync(this.path, "utf8");
            const dfxJson: DfxJson = JSON.parse(json);
            let dfxPackageJson = get_dfx_package_json();
            for (let [key, value] of Object.entries(dfxJson.canisters)) {
                let pack_config = dfxPackageJson.getCanister(key);
                if (pack_config !== undefined) {
                    value.pack_config = pack_config;
                }
                this.canisters.set(key, value);
            }
        }
    }
}

export const get_dfx_json = (): DfxJson => {
    return new DfxJsonFile();
}


export interface DfxPackageJson {
    canisters: Map<string, DfxPackageCanister>,
    envs: DfxPackageEnv[],

    getCanister(canister_id: string): DfxPackageCanister | undefined,
}


export interface DfxPackageCanister {
    exclude_in_package?: boolean,
}

export interface DfxPackageEnv {
    name: string,
    feature: string
}

export class FileDfxPackage implements DfxPackageJson {
    canisters: Map<string, DfxPackageCanister>;
    envs: DfxPackageEnv[];


    constructor(file_content: string) {
        this.canisters = new Map();
        const dfx_package_json = JSON.parse(file_content);
        for (let name in dfx_package_json["canisters"]) {
            this.canisters.set(name, dfx_package_json["canisters"][name] as DfxPackageCanister);
        }
        this.envs = [];
        for (const item of dfx_package_json["envs"]) {
            this.envs.push(item as DfxPackageEnv);
        }
    }

    getCanister(canister_id: string): DfxPackageCanister | undefined {
        return this.canisters.get(canister_id);
    }

}

export const get_dfx_package_json = (): DfxPackageJson => {
    let json = fs.readFileSync("./dfx_package.json", "utf8");
    return new FileDfxPackage(json);
}
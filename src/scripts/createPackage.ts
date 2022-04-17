import fs from "fs";
import archiver from "archiver";
import {DfxJsonCanister, DfxPackageEnv, get_dfx_json, get_dfx_package_json, get_wasm_path} from "~/utils/dfxJson";
import {canister} from "~/utils";
import logger from "node-color-log";

const package_dir = "package"
// dir to save packages build by diff feature
const package_features_dir = "package_feature"
const canister_ids_dir = "canister_ids"

const build_all = async (build_context: BuildContext) => {
    // reset package_feature dir
    if (fs.existsSync(package_features_dir)) {
        fs.rmSync(package_features_dir, {recursive: true})
    }
    fs.mkdirSync(package_features_dir)

    // distinct feature
    let features = build_context.features;

    // build each canister by each feature
    for (const feature of features) {
        // make a feature dif
        const feature_dir = `${package_features_dir}/${feature}`
        fs.mkdirSync(feature_dir)

        logger.debug(`build feature: ${feature}`);
        for (let [name, canister_json] of Object.entries(build_context.canisters)) {
            canister.build(name, feature);
            // copy wasm files to feature dir
            const wasm_path = get_wasm_path(canister_json);
            fs.copyFileSync(wasm_path, `${feature_dir}/${name}.wasm`);

            // copy did files to feature dir
            const did_path = canister_json.candid;
            fs.copyFileSync(did_path, `${feature_dir}/${name}.did`);
        }
    }
}

const clean = async () => {
    let found = fs.existsSync(package_dir);
    if (found) {
        logger.info("Cleaning package directory")
        fs.rmSync(package_dir, {recursive: true});
    }

    fs.mkdirSync(package_dir);
}

const check = async (build_context: BuildContext) => {
    // ensure every wasm file in package_feature dir must be < 2MB, check recursive
    for (let feature of build_context.features) {
        const feature_dir = `${package_features_dir}/${feature}`
        const files = fs.readdirSync(feature_dir);
        for (const file of files) {
            if (file.endsWith(".wasm")) {
                const file_path = `${feature_dir}/${file}`
                const stat = fs.statSync(file_path);
                if (stat.size > 2 * 1024 * 1024) {
                    throw new Error(`WASM file size of ${file} is ${stat.size} bytes, must be < 2MB`);
                }
            }
        }
    }

    logger.debug("Check passed")
}

const create = async (build_context: BuildContext) => {

    let out_dfx_json = {
        "defaults": {
            "build": {
                "args": "",
                "packtool": ""
            }
        },
        "networks": {
            "local": {
                "bind": "127.0.0.1:8000",
                "type": "ephemeral"
            },
            "ic": {
                "providers": ["https://ic0.app"],
                "type": "persistent"
            },
        },
        "version": 1
    };

    let canister_node = {};

    for (let name of Object.keys(build_context.canisters)) {
        canister_node[name] = {
            "candid": `assets/${name}.did`,
            "wasm": `assets/${name}.wasm`,
            "type": "custom"
        };
    }
    out_dfx_json["canisters"] = canister_node;

    logger.debug("creating package for each env");
    for (const env of build_context.envs) {
        logger.debug(`creating package for env: ${env.name}`);
        const env_dir = `${package_dir}/${env.name}`;
        fs.mkdirSync(env_dir);

        // copy canister_ids
        const source_canister_ids_json = `${canister_ids_dir}/${env.name}.json`;
        const dest_canister_ids_json = `${env_dir}/canister_ids.json`;
        fs.copyFileSync(source_canister_ids_json, dest_canister_ids_json);
        logger.debug(`copy canister_ids.json from ${source_canister_ids_json} to ${dest_canister_ids_json}`);

        // copy assets
        const env_assets_dir = `${env_dir}/assets`;
        fs.mkdirSync(env_assets_dir);

        // copy files from package_feature dir to env dir
        let feature = env.feature;
        const feature_dir = `${package_features_dir}/${feature}`;
        const files = fs.readdirSync(feature_dir);
        for (const file of files) {
            fs.copyFileSync(`${feature_dir}/${file}`, `${env_assets_dir}/${file}`);
        }
        logger.debug(`copy files from ${feature_dir} to ${env_assets_dir}`);

        // out dfx.json
        const dest_dfx_json = `${env_dir}/dfx.json`;
        fs.writeFileSync(dest_dfx_json, JSON.stringify(out_dfx_json, null, 2));
        logger.debug(`Created dfx.json for ${env.name}`);

        // create ${env}.env file
        const dest_env_file = `${env_dir}/${env.name}.env`;
        fs.writeFileSync(dest_env_file, env.name);
        logger.debug(`Created ${env.name}.env for ${env.name}`);

        logger.info(`Created package for ${env.name}`);
    }
}

const create_zip = async (build_context: BuildContext) => {
    // create zip file for each env
    for (const env of build_context.envs) {
        const env_dir = `${package_dir}/${env.name}`;
        let output_zip = fs.createWriteStream(`${env_dir}.zip`);
        let archive = archiver("zip", {
            zlib: {level: 9}
        });

        archive.pipe(output_zip);
        archive.directory(env_dir, false);
        await archive.finalize();

        logger.info(`Created zip file for ${env.name}`);
    }
}

interface BuildContext {
    canisters: Map<string, DfxJsonCanister>
    envs: DfxPackageEnv[],
    features: string[]
}

(async () => {
    let dfxJson = get_dfx_json();
    const dfxPackageJson = get_dfx_package_json();
    // join canisters keys as string
    let canisters_keys = Array.from(dfxJson.canisters.keys()).join(", ");
    logger.info(`There are canister listed in dfx.json: ${canisters_keys}`);

    // filter canister those not exclude in package
    let exclude_canisters: string[] = [];
    let canisters = {};
    // TODO: try to sort canisters by depends in dfx.json to make sure canister dependencies are loaded first

    for (let [name, canister] of dfxJson.canisters.entries()) {
        if (canister.pack_config?.exclude_in_package) {
            exclude_canisters.push(name);
            continue;
        }
        canisters[name] = canister;
    }

    if (exclude_canisters.length > 0) {
        logger.info(`Exclude canisters: ${exclude_canisters.join(", ")}`);
    }

    let build_context: BuildContext = {
        canisters: canisters as Map<string, DfxJsonCanister>,
        envs: dfxPackageJson.envs,
        features: [...new Set(dfxPackageJson.envs.map(env => env.feature))]
    };

    await clean();
    await build_all(build_context);
    await check(build_context);
    await create(build_context);
    await create_zip(build_context);

})().then(() => {
    logger.debug("Package created successfully");
})
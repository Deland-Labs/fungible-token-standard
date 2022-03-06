import {exec} from "shelljs";
import {favorites, registrar, registrar_control_gateway, registry, resolver} from "~/canisters/names";
import fs from "fs";
import logger from "node-color-log";


const download_did = async (canister) => {
    let result = exec(`dfx canister call ${canister} __get_candid_interface_tmp_hack`, {silent: true});
    if (result.code !== 0) {
        logger.error(result.stderr);
        process.exit(1);
    }
    let source_content = result.stdout;
    // substring from first " to last "
    let start = source_content.indexOf("\"") + 1;
    let end = source_content.lastIndexOf("\"");
    let did_content = source_content.substring(start, end);
    // replace \\n with \n
    did_content = did_content.replace(/\\n/g, "\n");
    return did_content;
};

(async () => {
    let names = [registrar, registrar_control_gateway, registry, favorites, resolver];
    for (let name of names) {
        let did_content = await download_did(name);
        let did_file = `canisters/${name}/src/${name}.did`;
        logger.debug(`Writing ${did_file}`);
        fs.writeFileSync(did_file, did_content);
    }

    logger.info("Did update complete");
})();
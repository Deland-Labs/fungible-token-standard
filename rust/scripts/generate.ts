import {exec} from "shelljs";
import {canisters} from "~/canisters";
import logger from "node-color-log";

(async () => {
    logger.debug("Generating code of canisters client ...");
    canisters.map(async ([name, config]) => {
        await exec(`dfx generate ${name}`, {silent: true});
    });

    // remove ./src/declarations/*/index.js
    await exec(`rm -rf ./src/declarations/*/index.js`);
    await exec(`rm -rf ./src/declarations/*/*.did`);
    // copy files from ./src/declarations/* to ./scripts/src/scripts/declarations/
    await exec(`cp -r ./src/declarations/* ./scripts/src/scripts/declarations/`);
    // remove ./src/declarations/*
    await exec(`rm -rf ./src`);
})().then(() => {
    logger.info("Generate complete");
});
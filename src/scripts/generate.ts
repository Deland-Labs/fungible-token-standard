import { exec } from "shelljs";
import { canisters } from "~/canisters";
import logger from "node-color-log";
import fs from "fs";

const download_did = async (canister) => {
  const command = `dfx canister call ${canister} __get_candid_interface_tmp_hack`;
  logger.debug(`download_did : ${command}`);
  let result = exec(command, { silent: true });
  if (result.code !== 0) {
    logger.error(`${canister} : ${result.stderr}`);
    process.exit(1);
  }
  let source_content = result.stdout;
  // substring from first " to last "
  let start = source_content.indexOf('"') + 1;
  let end = source_content.lastIndexOf('"');
  let did_content = source_content.substring(start, end);
  // replace \\n with \n
  did_content = did_content.replace(/\\n/g, "\n");
  return did_content;
};

(async () => {
  // for each canister
  canisters.map(async ([name, config]) => {
    let did_file = `${config.candid}`;
    logger.debug(` ${name}: did_file: ${did_file}`);
    let did_content = await download_did(name);
    fs.writeFileSync(did_file, did_content);
    await exec(`dfx generate ${name}`, { silent: true });
    logger.info("Generating");
  });

  // delay 10 seconds to allow for the files to be generated
  setTimeout(async () => {
    // remove ./src/declarations/*/index.js
    await exec(`rm -rf ./src/declarations/*/index.js`);
    await exec(`rm -rf ./src/declarations/*/*.did`);
    // copy files from ./src/declarations/* to ./scripts/src/scripts/declarations/
    await exec(
      `cp -r ./src/declarations/* ./scripts/src/scripts/declarations/`
    );
    // remove ./src/declarations/*
    await exec(`rm -rf ./src`);
    logger.info("Generate update complete");
  }, 3000);
})();

import {canister} from "~/utils";
import fs from "fs";
import logger from "node-color-log";

(async () => {
    await canister.create_all();
    const names = ["dft_basic"]

    logger.debug("local canister ids updated");

})();
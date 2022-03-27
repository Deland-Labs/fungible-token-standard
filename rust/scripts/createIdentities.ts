import "~/setup"

import {canister} from "~/utils";
import {addMainAsController} from "~/utils/canister";
import {createIdentities} from "~/utils/identity";
import fs from "fs";
import logger from "node-color-log";

createIdentities();
// identities.json written to disk
logger.debug("Identities created");

canister.createAll();
addMainAsController()
    .then(() => {
        logger.info("Main controller added");
    })
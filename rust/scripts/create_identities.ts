import "~/setup"

import {canister} from "~/utils";
import {addMainAsController} from "~/utils/canister";
import {createIdentities} from "~/utils/identity";
import fs from "fs";
import logger from "node-color-log";

createIdentities();
logger.debug("Identities created, identities.json written to disk");


canister.createAll();
addMainAsController()
    .then(() => {
        logger.info("Main controller added");
    })
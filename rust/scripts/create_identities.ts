import "~/setup"

import {identity, canister} from "~/utils";
import {add_main_as_controller} from "~/utils/canister";
import {create_identities, identities_to_json} from "~/utils/identity";
import fs from "fs";
import logger from "node-color-log";

create_identities();
const identities = identity.identities;
let identities_json = identities_to_json(identities)
// write json to identities.json
fs.writeFileSync("identities.json",identities_json);
logger.debug("Identities created, identities.json written to disk");


canister.create_all();
add_main_as_controller()
    .then(() => {
        logger.info("Main controller added");
    })
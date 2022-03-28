import {reinstall} from "~/canisters/ledger";
import logger from "node-color-log";

reinstall().then(() => {
    logger.info("Successfully reinstalled ledger");
}).catch((err) => {
    console.error("Failed to reinstall ledger", err);
});
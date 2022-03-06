import "~/setup"
import {reinstall_all} from "./src/tasks"
import logger from "node-color-log";

(async () => {
    await reinstall_all({
        build: true,
        init: true,
        canisters: {
            dft_basic: true,
            dft_burnable: true,
            dft_mintable: true,
            dft_receiver: true,
            dft_tx_storage: true,
        }
    });
})().then(() => {
    logger.info("reinstall_all.ts: All done.");
}).catch((err) => {
    console.error("reinstall_all.ts: Error:", err);
});
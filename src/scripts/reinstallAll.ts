import { reinstall_all } from "./src/tasks"
import logger from "node-color-log";

(async () => {
    throw new Error("Not implemented");
})().then(() => {
    logger.info("reinstall_all.ts: All done.");
}).catch((err) => {
    console.error("reinstall_all.ts: Error:", err);
});
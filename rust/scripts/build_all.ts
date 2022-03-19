import {canister} from "~/utils";

(async () => {
    await canister.createAll();
    canister.build_all();
})();


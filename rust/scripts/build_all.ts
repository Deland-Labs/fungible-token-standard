import {canister} from "~/utils";

(async () => {
    await canister.create_all();
    canister.build_all();
})();


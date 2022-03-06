import "../setup"
import {canister} from "../utils";
import {ReInstallOptions} from "~/utils/canister";
import {identities} from "~/utils/identity";


const build = () => {
    canister.build("dft_basic");
}

const reinstall_by_dfx = async () => {
    await canister.reinstall_code("dft_basic");

}
const init = () => {
}

export const reinstall = async (options?: ReInstallOptions) => {
    if (options?.build) {
        build();

    }
    await reinstall_by_dfx();

    if (options?.init) {
        init();
    }
}
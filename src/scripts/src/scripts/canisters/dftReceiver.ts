import "../setup"
import {canister} from "../utils";
import {ReInstallOptions} from "~/utils/canister";
const build = () => {
    canister.build("dft_receiver");
}

const reinstall_by_dfx = async () => {
    await canister.reinstall("dft_receiver");
}

export const reinstall = async (options?: ReInstallOptions) => {
    if (options?.build) {
        build();
    }
    await reinstall_by_dfx();
}
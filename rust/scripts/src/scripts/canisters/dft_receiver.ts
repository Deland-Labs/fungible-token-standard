import "../setup"
import {canister} from "../utils";
import {ReInstallOptions} from "~/utils/canister";
import {DFTInitOptions} from "../../tasks";
import {parseToCommon} from "~/utils/uint";
import BigNumber from "bignumber.js";
import logger from "node-color-log";

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
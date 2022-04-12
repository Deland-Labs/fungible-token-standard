import "../setup"
import {canister} from "../utils";
import {ReInstallOptions} from "~/utils/canister";
import {DFTInitOptions} from "../../tasks";
import {parseToCommon} from "~/utils/uint";
import BigNumber from "bignumber.js";
import logger from "node-color-log";

const build = () => {
    canister.build("dft_basic");
}

const reinstall_by_dfx = async (args: string) => {
    await canister.reinstall("dft_basic", args);
}

export const reinstall = async (options?: ReInstallOptions, initOption?: DFTInitOptions) => {
    if (options?.build) {
        build();
    }
    const name = initOption?.name ?? "Test Basic Token";
    const symbol = initOption?.symbol ?? "TBT";
    const decimals = initOption?.decimals ?? 18;
    const supply = new BigNumber(parseToCommon(initOption?.totalSupply ?? 100000000000000000000000000n)).toFixed();

    const fee = initOption?.fee ?? {
        rate: 0n,
        minimum: 1n,
        rate_decimals: 8,
    };
    const owner = initOption?.owner ? `opt principal "${initOption?.owner}"` : "null";
    const args = `'(null ,null ,"${name}", "${symbol}", ${decimals}:nat8, ${supply}:nat, record { minimum =${fee.minimum} : nat; rate = ${fee.rate} : nat; rateDecimals= ${fee.rate_decimals}:nat8 } , ${owner})'`;
    logger.debug(`Reinstall by dfx: ${args}`);
    await reinstall_by_dfx(args);
}
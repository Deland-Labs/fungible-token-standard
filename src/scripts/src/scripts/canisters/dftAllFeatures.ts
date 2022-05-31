import { ReInstallOptions } from "../utils/canister";
import { DFTInitOptions } from "../../tasks";
import { unit, canister } from "@deland-labs/ic-dev-kit";
import BigNumber from "bignumber.js";
import logger from "node-color-log";

const build = () => {
    canister.build("dft_all_features");
}

const reinstall_by_dfx = async (args: string) => {
    await canister.reinstall("dft_all_features", args);
}

export const reinstall = async (options?: ReInstallOptions, initOption?: DFTInitOptions) => {
    if (options?.build) {
        build();
    }
    const name = initOption?.name ?? "Test Basic Token2";
    const symbol = initOption?.symbol ?? "TBT2";
    const decimals = initOption?.decimals ?? 16;
    const supply = new BigNumber(unit.parseToCommon(initOption?.totalSupply ?? 1000000000000000000000000n)).toFixed();

    const fee = initOption?.fee ?? {
        rate: 0n,
        minimum: 1n,
        rate_decimals: 8,
    };
    const owner = initOption?.owner ? `opt principal "${initOption?.owner}"` : "null";
    const args = `'(null ,null ,"${name}", "${symbol}", ${decimals}:nat8, ${supply}:nat, record { minimum =${fee.minimum} : nat; rate = ${fee.rate} : nat32; rateDecimals= ${fee.rate_decimals}:nat8 } , ${owner},null)'`;
    logger.debug(`Reinstall by dfx: ${args}`);
    await reinstall_by_dfx(args);
}

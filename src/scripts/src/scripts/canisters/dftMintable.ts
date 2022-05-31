import { ReInstallOptions } from "../utils/canister";
import { DFTInitOptions } from "../../tasks";
import { unit, canister } from "@deland-labs/ic-dev-kit";
import BigNumber from "bignumber.js";
import logger from "node-color-log";

const build = () => {
    canister.build("dft_mintable");
}

const reinstall_by_dfx = async (args: string) => {
    await canister.reinstall("dft_mintable", args);
}

export const reinstall = async (options?: ReInstallOptions, initOption?: DFTInitOptions) => {
    if (options?.build) {
        build();
    }
    const name = initOption?.name ?? "Mintable Token";
    const symbol = initOption?.symbol ?? "MTT";
    const decimals = initOption?.decimals ?? 12;
    const supply = new BigNumber(unit.parseToCommon(initOption?.totalSupply ?? 100000000000000000000n)).toFixed();

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
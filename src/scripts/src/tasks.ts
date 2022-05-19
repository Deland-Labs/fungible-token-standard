import "./scripts/setup"
import {reinstall as reinstallDFTBasic} from "~/canisters/dftBasic";
import {reinstall as reinstallDFTBurnableMintale} from "~/canisters/dftAllFeatures";
import {reinstall as reinstallDFTBurnable} from "~/canisters/dfBurnable";
import {reinstall as reinstallDFTMintable} from "~/canisters/dftMintable";
import {reinstall as reinstallDFTTxStorage} from "~/canisters/dftTxStorage";
import {reinstall as reinstallDFTReceiver} from "~/canisters/dftReceiver";


export const reinstall_all = async (options?: CanisterReinstallOptions) => {
    // recode time of cost
    const start = Date.now();
    const jobs = Array<Promise<void>>();
    // dft basic
    if (options && options.canisters?.dft_basic) {
        jobs.push(reinstallDFTBasic({
            ...options,
        }, options.canisters.dft_basic.initOptions));
    }
    // dft basic 2
    if (options && options.canisters?.dft_all_features) {
        jobs.push(reinstallDFTBurnableMintale({...options,},
            options.canisters.dft_all_features.initOptions));
    }
    // dft burnable
    if (options && options.canisters?.dft_burnable) {
        jobs.push(reinstallDFTBurnable({...options,},
            options.canisters.dft_burnable.initOptions));
    }
    // dft mintable
    if (options && options.canisters?.dft_mintable) {
        jobs.push(reinstallDFTMintable({...options,},
            options.canisters.dft_mintable.initOptions));
    }

    // dfx receiver
    if (options && options.canisters?.dft_receiver) {
        jobs.push(reinstallDFTReceiver({...options,}));
    }

    // dfx tx storage
    if (options && options.canisters?.dft_tx_storage) {
        jobs.push(reinstallDFTTxStorage({...options,}));
    }
    if (options && options.one_by_one) {
        for (const task of jobs) {
            await task;
        }
    } else {
        console.info("reinstall all in parallel");
        await Promise.all(jobs);
    }

    const end = Date.now();
    console.info(`reinstall all in ${end - start} ms`);
    // sleep for 3 seconds to waiting code to be available
    await new Promise((resolve) => setTimeout(resolve, 3000));
}

export interface Fee {
    minimum: number,
    rate: number,
    rate_decimals: number
}

export interface DFTInitOptions {
    name: string;
    symbol: string;
    decimals: bigint;
    totalSupply: bigint;
    fee: Fee;
    desc: Array<[string, string]>;
    owner: string;
}

export interface CommonInstallOptions {
    reinstall: boolean;
}

export interface DFTInstallOptions extends CommonInstallOptions {
    initOptions: DFTInitOptions;
}

export interface CanisterReinstallOptionsCanisters {
    dft_basic?: DFTInstallOptions;
    dft_all_features?: DFTInstallOptions;
    dft_burnable?: DFTInstallOptions;
    dft_mintable?: DFTInstallOptions;
    dft_receiver?: CommonInstallOptions;
    dft_tx_storage?: CommonInstallOptions;
}

export interface CanisterReinstallOptions {
    build?: boolean;
    init?: boolean;
    one_by_one?: boolean;
    canisters?: CanisterReinstallOptionsCanisters;
}

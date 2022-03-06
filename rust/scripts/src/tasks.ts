import "./scripts/setup"
import {reinstall as reinstall_dft_basic} from "./scripts/canisters/dft_basic";


export const reinstall_all = async (options?: CanisterReinstallOptions) => {
    // recode time of cost
    const start = Date.now();

    if (options && options.one_by_one) {
        if (options && options.canisters?.dft_basic) {
            await reinstall_dft_basic({
                ...options,
            });
        }
    } else {
        console.info("reinstall all in parallel");
        let jobs: Promise<void>[] = [];
        if (options && options.canisters?.dft_basic) {
            jobs.push(reinstall_dft_basic({
                ...options,
            }));
        }

        await Promise.all(jobs);
    }

    const end = Date.now();
    console.info(`reinstall all in ${end - start} ms`);
    // sleep for 3 seconds to waiting code to be available
    await new Promise((resolve) => setTimeout(resolve, 3000));
}

export interface CanisterReinstallOptionsCanisters {
    dft_basic?: boolean;
    dft_burnable?: boolean;
    dft_mintable?: boolean;
    dft_receiver?: boolean;
    dft_tx_storage?: boolean;
}

export interface CanisterReinstallOptions {
    build?: boolean;
    init?: boolean;
    one_by_one?: boolean;
    canisters?: CanisterReinstallOptionsCanisters;
}
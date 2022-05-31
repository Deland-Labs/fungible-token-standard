import { canister } from "@deland-labs/ic-dev-kit";
import { ReInstallOptions } from "../utils/canister";
const build = () => {
    canister.build("dft_tx_storage");
}

const reinstall_by_dfx = async () => {
    const default_dft_id = "rrkah-fqaaa-aaaaa-aaaaq-cai"
    await canister.reinstall("dft_tx_storage", `'(principal "${default_dft_id}",1:nat)'`);
}

export const reinstall = async (options?: ReInstallOptions) => {
    if (options?.build) {
        build();
    }
    await reinstall_by_dfx();
}
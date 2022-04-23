import {createActor as createDFTBasic} from "~/declarations/dft_basic";
import {createActor as createDFTAllFeatures} from "~/declarations/dft_all_features";
import {createActor as createDFTBurnable} from "~/declarations/dft_burnable";
import {createActor as createDFTMintable} from "~/declarations/dft_mintable";
import {createActor as CreateStorageActor} from "~/declarations/dft_tx_storage";
import {identityFactory} from "~/utils/identity";
import {get_id} from "~/utils/canister";

const createDFTBasicActor = (user?: string) => {
    let canisterId = get_id("dft_basic");
    if (user === undefined) {
        return createDFTBasic(canisterId, {
            agentOptions: {host: identityFactory.getDefaultHost()},
        });
    }
    let identityInfo = identityFactory.getIdentity(user)!;
    return createDFTBasic(canisterId, {
        agentOptions: identityInfo.agentOptions,
    });
};

// create a dft_all_features actor
const createDFTWithAllFeatures = (user?: string) => {
    let canisterId = get_id("dft_all_features");
    if (user === undefined) {
        return createDFTAllFeatures(canisterId, {
            agentOptions: {host: identityFactory.getDefaultHost()},
        });
    }
    let identityInfo = identityFactory.getIdentity(user)!;
    return createDFTAllFeatures(canisterId, {
        agentOptions: identityInfo.agentOptions,
    });
};

// create a dft_burnable actor
const createDFTBurnableActor = (user?: string) => {
    let canisterId = get_id("dft_burnable");
    if (user === undefined) {
        return createDFTBurnable(canisterId, {
            agentOptions: {host: identityFactory.getDefaultHost()},
        });
    }
    let identityInfo = identityFactory.getIdentity(user)!;
    return createDFTBurnable(canisterId, {
        agentOptions: identityInfo.agentOptions,
    });
};

// create a mintable actor
const createDFTMintableActor = (user?: string) => {
    let canisterId = get_id("dft_mintable");
    if (user === undefined) {
        return createDFTMintable(canisterId, {
            agentOptions: {host: identityFactory.getDefaultHost()},
        });
    }
    let identityInfo = identityFactory.getIdentity(user)!;
    return createDFTMintable(canisterId, {
        agentOptions: identityInfo.agentOptions,
    });
};

// create tx storage actor
const createStorageActor = (canisterId: string) => {
    return CreateStorageActor(canisterId, {
        agentOptions: {host: identityFactory.getDefaultHost()},
    });
};

export {
    createDFTBasicActor,
    createDFTWithAllFeatures,
    createDFTBurnableActor,
    createDFTMintableActor,
    createStorageActor
};

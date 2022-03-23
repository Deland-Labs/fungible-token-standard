import {createActor as createDFTBasic} from "~/declarations/dft_basic";
import {createActor as createDFTBasic2} from "~/declarations/dft_basic2";
import {createActor as createDFTBurnable} from "~/declarations/dft_burnable";
import {createActor as createDFTMintable} from "~/declarations/dft_mintable";
import {identityFactory} from "~/utils/identity";
import {get_id} from "~/utils/canister";
import {assert} from "chai";

const createDFTBasicActor = (user: string) => {
    let identityInfo = identityFactory.getIdentity(user)!;
    assert(identityInfo !== undefined,   `Identity ${user} not found`);
    let canisterId = get_id("dft_basic");
    return createDFTBasic(canisterId, {agentOptions: identityInfo.agentOptions});
}

// create a dft_basic2 actor
const createDFTBasic2Actor = (user: string) => {
    let identityInfo = identityFactory.getIdentity(user)!;
    let canisterId = get_id("dft_basic2");
    return createDFTBasic2(canisterId, {agentOptions: identityInfo.agentOptions});
}

// create a dft_burnable actor
const createDFTBurnableActor = (user: string) => {
    let identityInfo = identityFactory.getIdentity(user)!;
    let canisterId = get_id("dft_burnable");
    return createDFTBurnable(canisterId, {agentOptions: identityInfo.agentOptions});
}

// create a mintable actor
const createDFTMintableActor = (user: string) => {
    let identityInfo = identityFactory.getIdentity(user)!;
    let canisterId = get_id("dft_mintable");
    return createDFTMintable(canisterId, {agentOptions: identityInfo.agentOptions});
}

export {
    createDFTBasicActor,
    createDFTBasic2Actor,
    createDFTBurnableActor,
    createDFTMintableActor
}



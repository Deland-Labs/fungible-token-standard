import "~/setup";
import {Given, Then, When} from "@cucumber/cucumber";
import {reinstall_all} from "../../src/tasks";
import {canister} from "~/utils";
import {
    createDFTBasic2Actor,
    createDFTBasicActor,
    createDFTBurnableActor,
    createDFTMintableActor
} from "~/declarations";

Then(/^Sleep for "([^"]*)" secs.$/, async function (sec: string) {
    // sleep for secs
    await new Promise(resolve => setTimeout(resolve, parseFloat(sec) * 1000));
});

export const reinstall_canisters = async (names: string[]): Promise<void> => {
    let canisters = {};
    for (const name of names) {
        canisters[name] = true;
    }

    console.info(`Reinstalling canisters: ${JSON.stringify(canisters)}`);

    await reinstall_all({
        build: false,
        init: true,
        canisters: canisters
    });
}

Given(/^Reinstall canisters$/,
    async function (data) {
        let target_canisters = data.hashes();
        let names: string[] = [];
        for (const item of target_canisters) {
            names.push(item.name);
        }
        await reinstall_canisters(names);
    });
When(/^canister "([^"]*)" is down$/, async function (canister_name: string) {
    await canister.uninstall_code(canister_name);
});

export const createDFTActor = (token, user) => {
    let actor = createDFTBasicActor(user);
    switch (token) {
        case "dft_basic":
            actor = createDFTBasicActor(user);
            break;
        case "dft_basic_2":
            actor = createDFTBasic2Actor(user);
            break;
        case "dft_burnable":
            actor = createDFTBurnableActor(user);
            break;
        case "dft_mintable":
            actor = createDFTMintableActor(user);
            break;
        default:
            break;
    }
    return actor;
}


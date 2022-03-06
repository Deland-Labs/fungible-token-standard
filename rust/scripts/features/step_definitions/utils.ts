import "~/setup";
import {Given, Then, When} from "@cucumber/cucumber";
import {toICPe8s} from "~/utils/convert";
import {identities} from "~/utils/identity";
import {reinstall_all} from "../../src/tasks";
import {expect} from "chai";
import {canister} from "~/utils";


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


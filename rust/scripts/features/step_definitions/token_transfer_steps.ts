import {Given, Then, When} from "@cucumber/cucumber";
import {assert, expect} from "chai";
import logger from "node-color-log";
import {CanisterReinstallOptions, DFTInitOptions, reinstall_all} from "../../src/tasks";
import {parseToOrigin} from "~/utils/uint";
import {
    createDFTBasic2Actor,
    createDFTBasicActor,
    createDFTBurnableActor,
    createDFTMintableActor
} from "~/declarations";
import {parseRawTableToJsonArray} from "~/utils/convert";
import {identityFactory} from "~/utils/identity";

When(/^Transfer from (.*) to (.*),(.*) (.*)$/, async function (userA, userB, diff, token) {
    logger.debug(`Transfer from ${userA} to ${userB},${diff} ${token}`);
    const userBPrincipal = identityFactory.getPrincipal(userB)!;
    switch (token) {
        case "dft_basic":
            const actor = createDFTBasicActor(userA);
            const decimals = await actor.decimals();
            const amountBN = parseToOrigin(diff, decimals);
            const res = await actor.transfer([], userBPrincipal.toText(), amountBN, []);
            assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);

            break;
        case "dft_basic2":
            const actor2 = createDFTBasic2Actor(userA);
            const decimals2 = await actor2.decimals();
            const amountBN2 = parseToOrigin(diff, decimals2);
            const res2 = await actor2.transfer([], userBPrincipal.toText(), amountBN2, []);
            assert.isTrue('Ok' in res2, `transfer failed: ${JSON.stringify(res2)}`);
            break;
        case "dft_burnable":
            const actor3 = createDFTBurnableActor(userA);
            const decimals3 = await actor3.decimals();
            const amountBN3 = parseToOrigin(diff, decimals3);
            const res3 = await actor3.transfer([], userBPrincipal.toText(), amountBN3, []);
            assert.isTrue('Ok' in res3, `transfer failed: ${JSON.stringify(res3)}`);
            break;
        case "dft_mintable":
            const actor4 = createDFTMintableActor(userA);
            const decimals4 = await actor4.decimals();
            const amountBN4 = parseToOrigin(diff, decimals4);
            const res4 = await actor4.transfer([], userBPrincipal.toText(), amountBN4, []);
            assert.isTrue('Ok' in res4, `transfer failed: ${JSON.stringify(res4)}`);
            break;
        default:
            break;
    }
});

Then(/^Check the (.*) balance of (.*) should be (.*)$/, async function (token, user, balance) {
    const userPrincipal = identityFactory.getPrincipal(user)!;
    switch (token) {
        case "dft_basic":
            const actor = createDFTBasicActor(user);
            const decimals = await actor.decimals();
            const balanceBN = parseToOrigin(balance, decimals);
            assert.equal(balanceBN, await actor.balanceOf(userPrincipal.toText()));
            break;
        case "dft_basic2":
            const actor2 = createDFTBasic2Actor(user);
            const decimals2 = await actor2.decimals();
            const balanceBN2 = parseToOrigin(balance, decimals2);
            assert.equal(balanceBN2, await actor2.balanceOf(userPrincipal.toText()));
            break;
        case "dft_burnable":
            const actor3 = createDFTBurnableActor(user);
            const decimals3 = await actor3.decimals();
            const balanceBN3 = parseToOrigin(balance, decimals3);
            assert.equal(balanceBN3, await actor3.balanceOf(userPrincipal.toText()));
            break;
        case "dft_mintable":
            const actor4 = createDFTMintableActor(user);
            const decimals4 = await actor4.decimals();
            const balanceBN4 = parseToOrigin(balance, decimals4);
            assert.equal(balanceBN4, await actor4.balanceOf(userPrincipal.toText()));
            break;
        default:
            break;
    }
});

Then(/^Check that the transfer fees of (.*) by (.*) charged fee is (.*)$/, function (token, transferAmount,fee) {


});
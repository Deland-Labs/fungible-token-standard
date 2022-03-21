import {Then, When} from "@cucumber/cucumber";
import {assert} from "chai";
import logger from "node-color-log";
import {parseToOrigin} from "~/utils/uint";
import {
    createDFTBasic2Actor,
    createDFTBasicActor,
    createDFTBurnableActor,
    createDFTMintableActor
} from "~/declarations";
import {identityFactory} from "~/utils/identity";

When(/^(.*) transfer (.*) (.*) to (.*) immediate$/, async function (userA, diff, token, userB) {
    logger.debug(`Transfer from ${userA} to ${userB},${diff} ${token}`);
    const userBPrincipal = identityFactory.getPrincipal(userB)!;
    let actor = createDFTBasicActor(userA);
    switch (token) {
        case "dft_basic":
            actor = createDFTBasicActor(userA);
            break;
        case "dft_basic2":
            actor = createDFTBasic2Actor(userA);
            break;
        case "dft_burnable":
            actor = createDFTBurnableActor(userA);
            break;
        case "dft_mintable":
            actor = createDFTMintableActor(userA);
            break;
        default:
            break;
    }
    const decimals = await actor.decimals();
    const amountBN = parseToOrigin(diff, decimals);
    const res = await actor.transfer([], userBPrincipal.toText(), amountBN, []);
    assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);
});

Then(/^Check the (.*) balance of (.*) should be (.*)$/, async function (token, user, balance) {
    const userPrincipal = identityFactory.getPrincipal(user)!;
    let actor = createDFTBasicActor(user);
    switch (token) {
        case "dft_basic":
            actor = createDFTBasicActor(user);
            break;
        case "dft_basic2":
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
    const decimals = await actor.decimals();
    const balanceBN = parseToOrigin(balance, decimals);
    assert.equal(balanceBN, await actor.balanceOf(userPrincipal.toText()));
});

Then(/^Check that the transfer fees of (.*) by (.*) charged fee is (.*),fee to is (.*)$/, async function (token, transferAmount, fee, feeTo) {
    const userPrincipal = identityFactory.getPrincipal(feeTo)!.toText();
    let actor = createDFTBasicActor(feeTo);
    switch (token) {
        case "dft_basic":
            actor = createDFTBasicActor(this.user);
            break;
        case "dft_basic2":
            actor = createDFTBasic2Actor(this.user);
            break;
        case "dft_burnable":
            actor = createDFTBurnableActor(this.user);
            break;
        case "dft_mintable":
            actor = createDFTMintableActor(this.user);
            break;
        default:
            break;
    }
    const decimals = await actor.decimals();
    const feeBN = parseToOrigin(fee, decimals);
    assert.equal(feeBN, await actor.balanceOf(userPrincipal));
});
When(/^(.*) transfer from (.*) to (.*),(.*) (.*)$/, async function (spender, owner, to, amount, token) {
    const userPrincipal = identityFactory.getPrincipal(owner)!;
    const toPrincipal = identityFactory.getPrincipal(to)!;
    let actor = createDFTBasicActor(owner);
    switch (token) {
        case "dft_basic":
            actor = createDFTBasicActor(spender);
            break;
        case "dft_basic2":
            actor = createDFTBasic2Actor(spender);
            break;
        case "dft_burnable":
            actor = createDFTBurnableActor(spender);
            break;
        case "dft_mintable":
            actor = createDFTMintableActor(spender);
            break;
        default:
            break;
    }
    const decimals = await actor.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor.transferFrom([], userPrincipal.toText(), toPrincipal.toText(), amountBN, []);
    assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);

});
Then(/^"([^"]*)" transfer "([^"]*)" from "([^"]*)" to "([^"]*)" "([^"]*)" will failed$/, async function (spender, token, owner, to, amount) {
    const ownerPrincipal = identityFactory.getPrincipal(owner)!.toText();
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    let actor = createDFTBasicActor(owner);
    switch (token) {
        case "dft_basic":
            actor = createDFTBasicActor(spender);
            break;
        case "dft_basic2":
            actor = createDFTBasic2Actor(spender);
            break;
        case "dft_burnable":
            actor = createDFTBurnableActor(spender);
            break;
        case "dft_mintable":
            actor = createDFTMintableActor(spender);
            break;
        default:
            break;
    }
    const decimals = await actor.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor.transferFrom([], ownerPrincipal, toPrincipal, amountBN, []);
    assert.isTrue('Err' in res, `transfer failed: ${JSON.stringify(res)}`);
});

Then(/^"([^"]*)" transfer "([^"]*)" from "([^"]*)" to "([^"]*)" "([^"]*)" will success$/, async function (spender, token, owner, to, amount) {
    const ownerPrincipal = identityFactory.getPrincipal(owner)!.toText();
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    let actor = createDFTBasicActor(owner);
    switch (token) {
        case "dft_basic":
            actor = createDFTBasicActor(spender);
            break;
        case "dft_basic2":
            actor = createDFTBasic2Actor(spender);
            break;
        case "dft_burnable":
            actor = createDFTBurnableActor(spender);
            break;
        case "dft_mintable":
            actor = createDFTMintableActor(spender);
            break;
        default:
            break;
    }
    const decimals = await actor.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor.transferFrom([], ownerPrincipal, toPrincipal, amountBN, []);
    assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);
});
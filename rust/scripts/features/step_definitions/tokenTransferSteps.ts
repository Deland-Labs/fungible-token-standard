import {Then, When} from "@cucumber/cucumber";
import {assert} from "chai";
import logger from "node-color-log";
import {parseToOrigin} from "~/utils/uint";
import {identityFactory} from "~/utils/identity";
import {createDFTActor} from "./utils";

When(/^(.*) transfer (.*) (.*) to (.*) immediate$/, async function (userA, diff, token, userB) {
    logger.debug(`Transfer from ${userA} to ${userB},${diff} ${token}`);
    const userBPrincipal = identityFactory.getPrincipal(userB)!;
    const actor = createDFTActor(token, userA);
    const decimals = await actor.decimals();
    const amountBN = parseToOrigin(diff, decimals);
    const res = await actor.transfer([], userBPrincipal.toText(), amountBN, []);
    assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);
});

Then(/^Check the (.*) balance of (.*) should be (.*)$/, async function (token, user, balance) {
    const userPrincipal = identityFactory.getPrincipal(user)!;
    const actor = createDFTActor(token, user);
    const decimals = await actor.decimals();
    const balanceBN = parseToOrigin(balance, decimals);
    const balanceRes = await actor.balanceOf(userPrincipal.toText());
    assert.equal(balanceBN, balanceRes, `balance of ${user} should be ${balanceRes}`);
});

Then(/^Check that the transfer fees of (.*) by (.*) charged fee is (.*),fee to is (.*)$/, async function (token, transferAmount, fee, feeTo) {
    const userPrincipal = identityFactory.getPrincipal(feeTo)!.toText();
    const actor = createDFTActor(token, feeTo);
    const decimals = await actor.decimals();
    const feeBN = parseToOrigin(fee, decimals);
    assert.equal(feeBN, await actor.balanceOf(userPrincipal));
});
When(/^(.*) transfer from (.*) to (.*),(.*) (.*)$/, async function (spender, owner, to, amount, token) {
    const userPrincipal = identityFactory.getPrincipal(owner)!;
    const toPrincipal = identityFactory.getPrincipal(to)!;
    const actor = createDFTActor(token, spender);
    const decimals = await actor.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor.transferFrom([], userPrincipal.toText(), toPrincipal.toText(), amountBN, []);
    assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);

});
Then(/^"([^"]*)" transfer "([^"]*)" from "([^"]*)" to "([^"]*)" "([^"]*)" will failed$/, async function (spender, token, owner, to, amount) {
    const ownerPrincipal = identityFactory.getPrincipal(owner)!.toText();
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const actor = createDFTActor(token, spender);
    const decimals = await actor.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor.transferFrom([], ownerPrincipal, toPrincipal, amountBN, []);
    assert.isTrue('Err' in res, `transfer failed: ${JSON.stringify(res)}`);
});

Then(/^"([^"]*)" transfer "([^"]*)" from "([^"]*)" to "([^"]*)" "([^"]*)" will success$/, async function (spender, token, owner, to, amount) {
    const ownerPrincipal = identityFactory.getPrincipal(owner)!.toText();
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const actor = createDFTActor(token, spender);
    const decimals = await actor.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor.transferFrom([], ownerPrincipal, toPrincipal, amountBN, []);
    assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);
});
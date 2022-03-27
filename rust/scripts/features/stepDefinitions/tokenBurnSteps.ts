import {Then, When} from "@cucumber/cucumber";
import {createDFTBurnableActor} from "~/declarations";
import {parseToOrigin} from "~/utils/uint";
import {assert} from "chai";
import {identityFactory} from "~/utils/identity";

When(/^"([^"]*)" burn (\d+) "([^"]*)" token$/, async function (user, amount, token) {
    const actor = createDFTBurnableActor(user);
    const decimals = await actor.decimals()
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor.burn([], amountBN, []);
    assert.isTrue("Ok" in res);
});
When(/^"([^"]*)" burn "([^"]*)" "([^"]*)" token will failed$/, async function (user, amount, token) {
    const actor = createDFTBurnableActor(user);
    const decimals = await actor.decimals()
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor.burn([], amountBN, []);
    assert.isTrue("Err" in res, `burn ${amount} ${token} token should failed: ${JSON.stringify(res)}`);
});

Then(/^Check that the fees of "([^"]*)" is "([^"]*)" by "([^"]*)", that means burn does not charge fee$/, async function (token, fee, user) {
    const actor = createDFTBurnableActor(user);
    const feeChargerPrincipal = identityFactory.getPrincipal(user)!.toText();
    const decimals = await actor.decimals()
    const feeBN = parseToOrigin(fee, decimals);
    const balanceBN = await actor.balanceOf(feeChargerPrincipal);
    assert.equal(balanceBN, feeBN);
});
Then(/^"([^"]*)" burn "([^"]*)" from "([^"]*)" "([^"]*)" token will failed$/, async function (spender, amount, owner, token) {
    const actor = createDFTBurnableActor(spender);
    const spenderPrincipal = identityFactory.getPrincipal(spender)!.toText();
    const ownerPrincipal = identityFactory.getPrincipal(owner)!.toText();
    const decimals = await actor.decimals()
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor.burnFrom([], ownerPrincipal, amountBN, []);
    assert.isTrue("Err" in res, `burn ${amount} ${token} token from ${owner} should failed: ${JSON.stringify(res)}`);
});
Then(/^"([^"]*)" burn "([^"]*)" from "([^"]*)" "([^"]*)" token will sucess$/, async function (spender, amount, owner, token) {
    const actor = createDFTBurnableActor(spender);
    const spenderPrincipal = identityFactory.getPrincipal(spender)!.toText();
    const ownerPrincipal = identityFactory.getPrincipal(owner)!.toText();
    const decimals = await actor.decimals()
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor.burnFrom([], ownerPrincipal, amountBN, []);
    assert.isTrue("Ok" in res, `burn ${amount} ${token} token from ${owner} should sucess: ${JSON.stringify(res)}`);
});
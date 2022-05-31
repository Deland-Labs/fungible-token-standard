import "./setup";
import { Then, When } from "@cucumber/cucumber";
import { assert, expect } from "chai";
import logger from "node-color-log";
import { createDFTActor } from "./utils";
import { unit, identity, canister } from "@deland-labs/ic-dev-kit";


When(/^(.*) transfer (.*) (.*) to (.*) immediate$/, async function (userA, diff, token, userB) {
    logger.debug(`Transfer from ${userA} to ${userB},${diff} ${token}`);
    const canisterReceiver = "dft_receiver";
    const userBPrincipal = userB === canisterReceiver ? canister.get_id(canisterReceiver) : identity.identityFactory.getPrincipal(userB)!.toText();
    const actor = createDFTActor(token, userA);
    const decimals = await actor!.decimals();
    const amountBN = unit.parseToOrigin(diff, decimals);
    const res = await actor!.transfer([], userBPrincipal, amountBN, []);
    assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);
});

Then(/^Check the (.*) balance of (.*) should be (.*)$/, async function (token, user, balance) {
    const canisterReceiver = "dft_receiver";
    const userBPrincipal = user === canisterReceiver ? canister.get_id(canisterReceiver) : identity.identityFactory.getPrincipal(user)!.toText();
    const actor = createDFTActor(token, user);
    const decimals = await actor!.decimals();

    const balanceBN = await actor!.balanceOf(userBPrincipal);
    const balanceRes = unit.parseToCommon(balanceBN, decimals);
    expect(balanceRes.toNumber()).to.equal(Number(balance));
});

Then(/^Check that the transfer fees of (.*) by (.*) charged fee is (.*),fee to is (.*)$/, async function (token, transferAmount, fee, feeTo) {
    const userPrincipal = identity.identityFactory.getPrincipal(feeTo)!.toText();
    const actor = createDFTActor(token, feeTo);
    const decimals = await actor!.decimals();
    const feeBN = unit.parseToOrigin(fee, decimals);
    assert.equal(feeBN, await actor!.balanceOf(userPrincipal));
});
When(/^(.*) transfer from (.*) to (.*),(.*) (.*)$/, async function (spender, owner, to, amount, token) {
    logger.debug(`Transfer from ${owner} to ${to},${amount} ${token}`);
    const userPrincipal = identity.identityFactory.getPrincipal(owner)!;
    const toPrincipal = identity.identityFactory.getPrincipal(to)!;
    const actor = createDFTActor(token, spender);
    const decimals = await actor!.decimals();
    const amountBN = unit.parseToOrigin(amount, decimals);
    const res = await actor!.transferFrom([], userPrincipal.toText(), toPrincipal.toText(), amountBN, []);
    assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);

});
Then(/^"([^"]*)" transfer "([^"]*)" from "([^"]*)" to "([^"]*)" "([^"]*)" will failed$/, async function (spender, token, owner, to, amount) {
    const ownerPrincipal = identity.identityFactory.getPrincipal(owner)!.toText();
    const toPrincipal = identity.identityFactory.getPrincipal(to)!.toText();
    const actor = createDFTActor(token, spender);
    const decimals = await actor!.decimals();
    const amountBN = unit.parseToOrigin(amount, decimals);
    const res = await actor!.transferFrom([], ownerPrincipal, toPrincipal, amountBN, []);
    assert.isTrue('Err' in res, `transfer failed: ${JSON.stringify(res)}`);
});

Then(/^"([^"]*)" transfer "([^"]*)" from "([^"]*)" to "([^"]*)" "([^"]*)" will success$/, async function (spender, token, owner, to, amount) {
    const ownerPrincipal = identity.identityFactory.getPrincipal(owner)!.toText();
    const toPrincipal = identity.identityFactory.getPrincipal(to)!.toText();
    const actor = createDFTActor(token, spender);
    const decimals = await actor!.decimals();
    const amountBN = unit.parseToOrigin(amount, decimals);
    logger.debug(`${spender} Transfer from ${owner} to ${to},${amount} ${token}`);
    const res = await actor!.transferFrom([], ownerPrincipal, toPrincipal, amountBN, []);
    assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);
});
When(/^"([^"]*)" transfer "([^"]*)" from "([^"]*)" to "([^"]*)" "([^"]*)" twice, the second will failed$/, async function (spender, token, owner, to, amount) {
    const ownerPrincipal = identity.identityFactory.getPrincipal(owner)!.toText();
    const toPrincipal = identity.identityFactory.getPrincipal(to)!.toText();
    const actor = createDFTActor(token, spender);
    const decimals = await actor!.decimals();
    const amountBN = unit.parseToOrigin(amount, decimals);
    //  set created_at as nanos timestamp
    const created_at = BigInt(new Date().getTime()) * 1000000n;
    logger.debug(`${spender} transfer from ${owner} to ${to},${amount} ${token}`);
    const res = await actor!.transferFrom([], ownerPrincipal, toPrincipal, amountBN, [created_at]);
    assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);
    const res2 = await actor!.transferFrom([], ownerPrincipal, toPrincipal, amountBN, [created_at]);
    assert.isTrue('Err' in res2, `transfer failed: ${JSON.stringify(res2)}`);
});

When(/^"([^"]*)" transfer "([^"]*)" "([^"]*)" to "([^"]*)" twice, the second will fail$/, async function (from, amount, token, to) {
    const fromPrincipal = identity.identityFactory.getPrincipal(from)!.toText();
    const toPrincipal = identity.identityFactory.getPrincipal(to)!.toText();
    const actor = createDFTActor(token, from);
    const decimals = await actor!.decimals();
    //  set created_at as nanos timestamp
    const created_at = BigInt(new Date().getTime()) * 1000000n;
    const amountBN = unit.parseToOrigin(amount, decimals);
    logger.debug(`${from} Transfer from ${from} to ${to},${amount} ${token}`);
    const res = await actor!.transfer([], toPrincipal, amountBN, [created_at]);
    assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);
    const res2 = await actor!.transfer([], toPrincipal, amountBN, [created_at]);
    assert.isTrue('Err' in res2, `transfer succeed: ${JSON.stringify(res2)}`);
});
When(/^"([^"]*)" transfer "([^"]*)" "([^"]*)" to "([^"]*)" passed "(\d+)" days will fail$/, async function (from, amount, token, to, passedDays) {
    const toPrincipal = identity.identityFactory.getPrincipal(to)!.toText();
    const actor = createDFTActor(token, from);
    const decimals = await actor!.decimals();
    //  set created_at as nanos timestamp
    const created_at = BigInt(new Date().getTime()) * 1000000n;
    const passedNanos = created_at - BigInt(passedDays) * 24n * 60n * 60n * 1000000000n;
    const amountBN = unit.parseToOrigin(amount, decimals);
    logger.debug(`${from} Transfer from ${from} to ${to},${amount} ${token}`);
    const res = await actor!.transfer([], toPrincipal, amountBN, [passedNanos]);
    assert.isTrue('Err' in res, `transfer succeed: ${JSON.stringify(res)}`);
});
When(/^"([^"]*)" transfer "([^"]*)" from "([^"]*)" to "([^"]*)" "([^"]*)" with timestamp passed "([^"]*)" day, will failed$/, async function (spender, token, owner, to, amount, passedDays) {
    const ownerPrincipal = identity.identityFactory.getPrincipal(owner)!.toText();
    const toPrincipal = identity.identityFactory.getPrincipal(to)!.toText();
    const actor = createDFTActor(token, spender);
    const decimals = await actor!.decimals();
    //  set created_at as nanos timestamp
    const created_at = BigInt(new Date().getTime()) * 1000000n;
    const passedNanos = created_at - BigInt(passedDays) * 24n * 60n * 60n * 1000000000n;
    const amountBN = unit.parseToOrigin(amount, decimals);
    logger.debug(`${spender} transfer from ${owner} to ${to},${amount} ${token}`);
    const res = await actor!.transferFrom([], ownerPrincipal, toPrincipal, amountBN, [passedNanos]);
    assert.isTrue('Err' in res, `transfer succeed: ${JSON.stringify(res)}`);
});
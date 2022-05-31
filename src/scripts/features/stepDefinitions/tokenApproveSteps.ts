import "./setup";
import { Given, When, Then, DataTable } from "@cucumber/cucumber";
import { unit, identity } from "@deland-labs/ic-dev-kit";
import { assert } from "chai";
import { createDFTActor } from "./utils";

Given(/^approve tokens from owner to spender in table$/, async function (dataTable: DataTable) {
    const optionArray = dataTable.hashes();
    for (let i = 0; i < optionArray.length; i++) {
        const option = optionArray[i];
        const dftActor = createDFTActor(option.token, option.owner);
        const owner = identity.identityFactory.getPrincipal(option.owner)!.toText();
        const decimals = await dftActor!.decimals();
        const spender = identity.identityFactory.getPrincipal(option.spender)!.toText();
        const amountBN = unit.parseToOrigin(option.amount, decimals);
        const res = await dftActor!.approve([], spender, amountBN, []);
        assert.isTrue('Ok' in res, `approve failed: ${JSON.stringify(res)}`);
        assert.equal(await dftActor!.allowance(owner, spender), amountBN);
    }
});

When(/^"(.*)" approve "(.*)" to "(.*)", "(.*)"$/, async function (owner, token, spender, amount) {
    const ownerId = identity.identityFactory.getPrincipal(owner)!.toText();
    const spenderId = identity.identityFactory.getPrincipal(spender)!.toText();
    const actor = createDFTActor(token, owner);
    const decimals = await actor!.decimals();
    const amountBN = unit.parseToOrigin(amount, decimals);
    const res = await actor!.approve([], spenderId, amountBN, []);
    assert.isTrue('Ok' in res, `approve failed: ${JSON.stringify(res)}`);
    assert.equal(await actor!.allowance(ownerId, spenderId), amountBN);
});

Then(/^Check the "(.*)" allowance of "(.*)" "(.*)" should be "(.*)"$/, async function (token, owner, spender, newAmount) {
    const ownerId = identity.identityFactory.getPrincipal(owner)!.toText();
    const spenderId = identity.identityFactory.getPrincipal(spender)!.toText();
    const actor = createDFTActor(token, owner);
    const decimals = await actor!.decimals();
    const newAmountBN = unit.parseToOrigin(newAmount, decimals);
    const res = await actor!.allowance(ownerId, spenderId);
    assert.equal(res, newAmountBN);
});
When(/^"([^"]*)" approve "([^"]*)" to "([^"]*)", "([^"]*)" twice , the second will failed$/, async function (owner, token, spender, amount
) {
    const ownerId = identity.identityFactory.getPrincipal(owner)!.toText();
    const spenderId = identity.identityFactory.getPrincipal(spender)!.toText();
    const actor = createDFTActor(token, owner);
    const decimals = await actor!.decimals();
    const amountBN = unit.parseToOrigin(amount, decimals);
    //  set created_at as nanos timestamp
    const created_at = BigInt(new Date().getTime()) * 1000000n;

    const res = await actor!.approve([], spenderId, amountBN, [created_at]);
    assert.isTrue('Ok' in res, `approve failed: ${JSON.stringify(res)}`);
    assert.equal(await actor!.allowance(ownerId, spenderId), amountBN);
    const res2 = await actor!.approve([], spenderId, amountBN, [created_at]);
    assert.isTrue('Err' in res2, `approve succeed: ${JSON.stringify(res2)}`);
});
When(/^"([^"]*)" approve "([^"]*)" to "([^"]*)", "([^"]*)" with timestamp passed "([^"]*)" day, will failed$/, async function (owner, token, spender, amount, passedDays) {
    const spenderId = identity.identityFactory.getPrincipal(spender)!.toText();
    const actor = createDFTActor(token, owner);
    const decimals = await actor!.decimals();
    const amountBN = unit.parseToOrigin(amount, decimals);
    //  set created_at as nanos timestamp
    const created_at_nanos = BigInt(new Date().getTime()) * 1000000n;

    const passed_days = BigInt(passedDays);
    const passed_time = created_at_nanos - passed_days * 24n * 60n * 60n * 1000000000n;
    const res = await actor!.approve([], spenderId, amountBN, [passed_time]);
    assert.isTrue('Err' in res, `approve failed: ${JSON.stringify(res)}`);
});
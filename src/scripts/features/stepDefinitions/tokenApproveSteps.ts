import {Given, When, Then} from "@cucumber/cucumber";
import {parseRawTableToJsonArray} from "~/utils/convert";
import {identityFactory} from "~/utils/identity";
import {parseToOrigin} from "~/utils/uint";
import {assert} from "chai";
import {createDFTActor} from "./utils";

Given(/^approve tokens from owner to spender in table$/, async function ({rawTable}) {
    const optionArray = parseRawTableToJsonArray(rawTable);
    for (let i = 0; i < optionArray.length; i++) {
        const option = optionArray[i];
        const dftActor = createDFTActor(option.token, option.owner);
        const owner = identityFactory.getPrincipal(option.owner)!.toText();
        if (dftActor && option) {
            const decimals = await dftActor.decimals();
            const spender = identityFactory.getPrincipal(option.spender)!.toText();
            const amountBN = parseToOrigin(option.amount, decimals);
            const res = await dftActor.approve([], spender, amountBN, []);
            assert.isTrue('Ok' in res, `approve failed: ${JSON.stringify(res)}`);
            assert.equal(await dftActor.allowance(owner, spender), amountBN);
        }
    }
});

When(/^"(.*)" approve "(.*)" to "(.*)", "(.*)"$/, async function (owner, token, spender, amount) {
    const ownerId = identityFactory.getPrincipal(owner)!.toText();
    const spenderId = identityFactory.getPrincipal(spender)!.toText();
    const actor = createDFTActor(token, owner);
    if (actor && spenderId && amount) {
        const decimals = await actor.decimals();
        const amountBN = parseToOrigin(amount, decimals);
        const res = await actor.approve([], spenderId, amountBN, []);
        assert.isTrue('Ok' in res, `approve failed: ${JSON.stringify(res)}`);
        assert.equal(await actor.allowance(ownerId, spenderId), amountBN);
    }
});

Then(/^Check the "(.*)" allowance of "(.*)" "(.*)" should be "(.*)"$/, async function (token, owner, spender, newAmount) {
    const ownerId = identityFactory.getPrincipal(owner)!.toText();
    const spenderId = identityFactory.getPrincipal(spender)!.toText();
    const actor = createDFTActor(token, owner);
    if (actor && spenderId && newAmount) {
        const decimals = await actor.decimals();
        const newAmountBN = parseToOrigin(newAmount, decimals);
        const res = await actor.allowance(ownerId, spenderId);
        assert.equal(res, newAmountBN);
    }
});
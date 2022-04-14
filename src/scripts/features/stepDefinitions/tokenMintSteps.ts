import {When, Then} from "@cucumber/cucumber";
import {assert, expect} from "chai";
import {createDFTMintableActor} from "~/declarations";
import {identityFactory} from "~/utils/identity";
import {parseToOrigin} from "~/utils/uint";

When(/^"([^"]*)" mint (\d+) "([^"]*)" for "([^"]*)" token will fail$/, async function (notOwner, amount, token, mintTo) {
    const actor = createDFTMintableActor(notOwner);
    const mintToPrincipal = identityFactory.getPrincipal(mintTo)!.toText();
    const decimals = await actor.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    try {
        const res = await actor.mint(mintToPrincipal, amountBN, []);
        expect.fail(`Minting token by not owner should fail`);
    } catch (e) {
        this.result = e;
    }
});

When(/^"([^"]*)" mint (\d+) "([^"]*)" for "([^"]*)" token will success$/, async function (notOwner, amount, token, mintTo) {
    const actor = createDFTMintableActor(notOwner);
    const mintToPrincipal = identityFactory.getPrincipal(mintTo)!.toText();
    const decimals = await actor.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor.mint(mintToPrincipal, amountBN, []);
    assert.isTrue("Ok" in res,`Minting token should success, but ${JSON.stringify(res)}`);
});

Then(/^Check that the fees of "([^"]*)" is "([^"]*)" by "([^"]*)", that means mint does not charge fee$/, async function (token, fee, user) {
    const actor = createDFTMintableActor(user);
    const feeChargerPrincipal = identityFactory.getPrincipal(user)!.toText();
    const decimals = await actor.decimals()
    const feeBN = parseToOrigin(fee, decimals);
    const balanceBN = await actor.balanceOf(feeChargerPrincipal);
    assert.equal(balanceBN, feeBN);
});

Then(/^Check the total supply of "([^"]*)" should be "([^"]*)"$/, async function (token, supply) {
    const user = "dft_main"
    const actor = createDFTMintableActor(user);
    const decimals = await actor.decimals();
    const supplyBN = parseToOrigin(supply, decimals);
    const totalSupplyBN = await actor.totalSupply();
    assert.equal(totalSupplyBN, supplyBN);
});
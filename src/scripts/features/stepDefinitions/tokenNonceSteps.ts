import {Then, When} from "@cucumber/cucumber";
import {createDFTActor, fileToByteArray} from "./utils";
import {identityFactory} from "~/utils/identity";
import {assert, expect} from "chai";
import {parseRawTableToJsonArray} from "~/utils/convert";
import {Fee} from "~/declarations/dft_basic/dft_basic.did";
import {parseToOrigin} from "~/utils/uint";
import {createDFTBurnableActor, createDFTMintableActor} from "~/declarations";
import logger from "node-color-log";

When(/^"([^"]*)" approve "([^"]*)" to "([^"]*)", "([^"]*)" with wrong nonce , will fail$/, async function (owner, token, sender, amount) {
    const actor = createDFTActor(token, owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const wrongNonce = crtNonce + 2n;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.approve([], sender, amountBN, [wrongNonce]);
    assert.isTrue("Err" in res);
    const updatedNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(updatedNonce, crtNonce);
});
When(/^"([^"]*)" approve "([^"]*)" to "([^"]*)", "([^"]*)" with out nonce , the nonce should increase (\d+)$/, async function (owner, token, spender, amount, increase) {
    const actor = createDFTActor(token, owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const spenderPrincipal = identityFactory.getPrincipal(spender)!.toText();
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const increaseBN = BigInt(increase);
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.approve([], spenderPrincipal, amountBN, []);
    assert.isTrue("Ok" in res, `${JSON.stringify(res)}`);
    const newNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(newNonce, crtNonce + increaseBN);
});
When(/^"([^"]*)" approve "([^"]*)" to "([^"]*)", "([^"]*)" with correct nonce , the nonce should increase (\d+)$/, async function (owner, token, spender, amount, increase) {
    const actor = createDFTActor(token, owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const spenderPrincipal = identityFactory.getPrincipal(spender)!.toText();
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.approve([], spenderPrincipal, amountBN, [newNonce]);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(updatedNonce, newNonce);
});

Then(/^"([^"]*)" transfer "([^"]*)" from "([^"]*)" to "([^"]*)" "([^"]*)" will fail, the nonce will not change$/, async function (spender, token, owner, to, amount) {
    const actor = createDFTActor(token, spender);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!.toText();
    const spenderPrincipal = identityFactory.getPrincipal(spender)!;
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const crtNonce = await actor!.nonceOf(spenderPrincipal);
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.transferFrom([], ownerPrincipal, toPrincipal, amountBN, []);
    assert.isTrue("Err" in res, `transfer from ${owner} to ${to} with amount ${amount} should failed ,but it succeed , ${JSON.stringify(res)}`);
    const updatedNonce = await actor!.nonceOf(spenderPrincipal);
    assert.equal(updatedNonce, crtNonce);

});
Then(/^"([^"]*)" transfer "([^"]*)" from "([^"]*)" to "([^"]*)" "([^"]*)" with wrong nonce will fail, the nonce will not change$/, async function (spender, token, owner, to, amount) {
    const actor = createDFTActor(token, spender);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!.toText();
    const spenderPrincipal = identityFactory.getPrincipal(spender)!;
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const crtNonce = await actor!.nonceOf(spenderPrincipal);
    const wrongNonce = crtNonce + 2n;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.transferFrom([], ownerPrincipal, toPrincipal, amountBN, [wrongNonce]);
    assert.isTrue("Err" in res);
    const updatedNonce = await actor!.nonceOf(spenderPrincipal);
    assert.equal(updatedNonce, crtNonce);
});

Then(/^"([^"]*)" transfer "([^"]*)" from "([^"]*)" to "([^"]*)" "([^"]*)" without nonce will success, the nonce should increase (\d+)$/, async function (spender, token, owner, to, amount, increase) {
    const actor = createDFTActor(token, spender);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!.toText();
    const spenderPrincipal = identityFactory.getPrincipal(spender)!;
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const crtNonce = await actor!.nonceOf(spenderPrincipal);
    const increaseBN = BigInt(increase);
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.transferFrom([], ownerPrincipal, toPrincipal, amountBN, []);
    assert.isTrue("Ok" in res);
    const newNonce = await actor!.nonceOf(spenderPrincipal);
    assert.equal(newNonce, crtNonce + increaseBN);
});

Then(/^"([^"]*)" transfer "([^"]*)" from "([^"]*)" to "([^"]*)" "([^"]*)" with correct nonce will success, the nonce should increase (\d+)$/, async function (spender, token, owner, to, amount, increase) {
    const actor = createDFTActor(token, spender);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!.toText();
    const spenderPrincipal = identityFactory.getPrincipal(spender)!;
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const crtNonce = await actor!.nonceOf(spenderPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.transferFrom([], ownerPrincipal, toPrincipal, amountBN, [newNonce]);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(spenderPrincipal);
    assert.equal(updatedNonce, newNonce);
});

When(/^"([^"]*)" transfer "([^"]*)" "([^"]*)" to "([^"]*)" with out nonce,will fail, the nonce will not change$/, async function (from, amount, token, to) {
    const actor = createDFTActor(token, from);
    const fromPrincipal = identityFactory.getPrincipal(from)!;
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const crtNonce = await actor!.nonceOf(fromPrincipal);
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.transfer([], toPrincipal, amountBN, []);
    assert.isTrue("Err" in res);
    const updatedNonce = await actor!.nonceOf(fromPrincipal);
    assert.equal(updatedNonce, crtNonce);
});
When(/^"([^"]*)" transfer "([^"]*)" "([^"]*)" to "([^"]*)" with wrong nonce,will fail, the nonce will not change$/, async function (from, amount, token, to) {
    const actor = createDFTActor(token, from);
    const fromPrincipal = identityFactory.getPrincipal(from)!;
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const crtNonce = await actor!.nonceOf(fromPrincipal);
    const wrongNonce = crtNonce + 2n;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.transfer([], toPrincipal, amountBN, [wrongNonce]);
    assert.isTrue("Err" in res);
    const updatedNonce = await actor!.nonceOf(fromPrincipal);
    assert.equal(updatedNonce, crtNonce);
});
When(/^"([^"]*)" transfer "([^"]*)" "([^"]*)" to "([^"]*)" with out nonce, the nonce should increase (\d+)$/, async function (from, amount, token, to, increase) {
    const actor = createDFTActor(token, from);
    const fromPrincipal = identityFactory.getPrincipal(from)!;
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const crtNonce = await actor!.nonceOf(fromPrincipal);
    const increaseBN = BigInt(increase);
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.transfer([], toPrincipal, amountBN, []);
    assert.isTrue("Ok" in res);
    const newNonce = await actor!.nonceOf(fromPrincipal);
    assert.equal(newNonce, crtNonce + increaseBN);
});
When(/^"([^"]*)" transfer "([^"]*)" "([^"]*)" to "([^"]*)" with correct nonce, the nonce should increase (\d+)$/, async function (from, amount, token, to, increase) {
    const actor = createDFTActor(token, from);
    const fromPrincipal = identityFactory.getPrincipal(from)!;
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const crtNonce = await actor!.nonceOf(fromPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.transfer([], toPrincipal, amountBN, [newNonce]);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(fromPrincipal);
    assert.equal(updatedNonce, newNonce);
});

When(/^I update token "([^"]*)"'s logo "([^"]*)" with owner "([^"]*)" with wrong nonce, will fail, the nonce will not change$/, async function (token, logoName, owner) {
    const actor = createDFTActor(token, owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const wrongNonce = crtNonce + 2n;
    const logoData = fileToByteArray(`./scripts/assets/${logoName}`);
    const logoParam: [number[]] = [Array.from(logoData)];
    const res = await actor!.setLogo(logoParam, [wrongNonce]);
    assert.isTrue("Err" in res);
    const updatedNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(updatedNonce, crtNonce);
});

When(/^I update token "([^"]*)"'s logo "([^"]*)" with owner "([^"]*)" with out nonce, the nonce should increase (\d+)$/, async function (token, logoName, owner, increase) {
    const actor = createDFTActor(token, owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const logoData = fileToByteArray(`./scripts/assets/${logoName}`);
    const logoParam: [number[]] = [Array.from(logoData)];
    const increaseBN = BigInt(increase);
    const res = await actor!.setLogo(logoParam, []);
    assert.isTrue("Ok" in res);
    const newNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(newNonce, crtNonce + increaseBN);
});

When(/^I update token "([^"]*)"'s logo "([^"]*)" with owner "([^"]*)" with correct nonce, the nonce should increase (\d+)$/, async function (token, logoName, owner, increase) {
    const actor = createDFTActor(token, owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const logoData = fileToByteArray(`./scripts/assets/${logoName}`);
    const logoParam: [number[]] = [Array.from(logoData)];
    const res = await actor!.setLogo(logoParam, [newNonce]);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(updatedNonce, newNonce);
});

When(/^I update token "([^"]*)"'s description with not owner "([^"]*)", will fail, the nonce will not change$/, async function (token, notOwner, {rawTable}) {
    const actor = createDFTActor(token, notOwner);
    const notOwnerPrincipal = identityFactory.getPrincipal(notOwner)!;
    const crtNonce = await actor!.nonceOf(notOwnerPrincipal);
    const optionArray = parseRawTableToJsonArray(rawTable);
    let desc: Array<[string, string]> = [];
    //convert optionArray to desc
    for (let i = 0; i < optionArray.length; i++) {
        desc.push([optionArray[i].key, optionArray[i].value]);
    }
    try {
        const res = await actor!.setDesc(desc, []);
        expect.fail(`Update description without owner should fail, but succeed with ${res}`);
    } catch (e) {
    }
    const updatedNonce = await actor!.nonceOf(notOwnerPrincipal);
    assert.equal(updatedNonce, crtNonce);
});

When(/^I update token "([^"]*)"'s description with owner "([^"]*)" with wrong nonce, will fail, the nonce will not change$/, async function (token, notOwner, {rawTable}) {
    const actor = createDFTActor(token, notOwner);
    const notOwnerPrincipal = identityFactory.getPrincipal(notOwner)!;
    const crtNonce = await actor!.nonceOf(notOwnerPrincipal);
    const wrongNonce = crtNonce + 2n;
    const optionArray = parseRawTableToJsonArray(rawTable);
    let desc: Array<[string, string]> = [];
    //convert optionArray to desc
    for (let i = 0; i < optionArray.length; i++) {
        desc.push([optionArray[i].key, optionArray[i].value]);
    }
    const res = await actor!.setDesc(desc, [wrongNonce]);
    assert.isTrue("Err" in res);
    const updatedNonce = await actor!.nonceOf(notOwnerPrincipal);
    assert.equal(updatedNonce, crtNonce);
});

When(/^I update token "([^"]*)"'s description with owner "([^"]*)" with out nonce, the nonce should increase (\d+)$/, async function (token, notOwner, increase, {rawTable}) {
    const actor = createDFTActor(token, notOwner);
    const notOwnerPrincipal = identityFactory.getPrincipal(notOwner)!;
    const crtNonce = await actor!.nonceOf(notOwnerPrincipal);
    const optionArray = parseRawTableToJsonArray(rawTable);
    const increaseBN = BigInt(increase);
    let desc: Array<[string, string]> = [];
    //convert optionArray to desc
    for (let i = 0; i < optionArray.length; i++) {
        desc.push([optionArray[i].key, optionArray[i].value]);
    }
    const res = await actor!.setDesc(desc, []);
    assert.isTrue("Ok" in res);
    const newNonce = await actor!.nonceOf(notOwnerPrincipal);
    assert.equal(newNonce, crtNonce + increaseBN);
});
When(/^I update token "([^"]*)"'s description with owner "([^"]*)" with correct nonce, the nonce should increase (\d+)$/, async function (token, notOwner, increase, {rawTable}) {
    const actor = createDFTActor(token, notOwner);
    const notOwnerPrincipal = identityFactory.getPrincipal(notOwner)!;
    const crtNonce = await actor!.nonceOf(notOwnerPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const optionArray = parseRawTableToJsonArray(rawTable);
    let desc: Array<[string, string]> = [];
    //convert optionArray to desc
    for (let i = 0; i < optionArray.length; i++) {
        desc.push([optionArray[i].key, optionArray[i].value]);
    }
    const res = await actor!.setDesc(desc, [newNonce]);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(notOwnerPrincipal);
    assert.equal(updatedNonce, newNonce);
});

When(/^I update token "([^"]*)"'s fee with owner "([^"]*)" with wrong nonce, will fail, the nonce will not change$/, async function (token, owner, {rawTable}) {
    const actor = createDFTActor(token, owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const wrongNonce = crtNonce + 2n;
    const optionArray = parseRawTableToJsonArray(rawTable);
    const option = optionArray[0];
    const decimals = await actor!.decimals();
    // convert optionArray to Fee
    const fee: Fee = {
        minimum: parseToOrigin(option.minimum, decimals),
        rate: parseToOrigin(option.rate, option.rate_decimals),
        rate_decimals: Number(option.rate_decimals)
    };
    const res = await actor!.setFee(fee, [wrongNonce]);
    assert.isTrue("Err" in res);
    const updatedNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(updatedNonce, crtNonce);
});
When(/^I update token "([^"]*)"'s fee with owner "([^"]*)" with out nonce, the nonce should increase (\d+)$/, async function (token, owner, increase, {rawTable}) {
    const actor = createDFTActor(token, owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const optionArray = parseRawTableToJsonArray(rawTable);
    const option = optionArray[0];
    const decimals = await actor!.decimals();
    // convert optionArray to Fee
    const fee: Fee = {
        minimum: parseToOrigin(option.minimum, decimals),
        rate: parseToOrigin(option.rate, option.rate_decimals),
        rate_decimals: Number(option.rate_decimals)
    };
    const res = await actor!.setFee(fee, []);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(updatedNonce, newNonce);
});

When(/^I update token "([^"]*)"'s fee with owner "([^"]*)" with correct nonce, the nonce should increase (\d+)$/, async function (token, owner, increase, {rawTable}) {
    const actor = createDFTActor(token, owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const optionArray = parseRawTableToJsonArray(rawTable);
    const option = optionArray[0];
    const decimals = await actor!.decimals();
    // convert optionArray to Fee
    const fee: Fee = {
        minimum: parseToOrigin(option.minimum, decimals),
        rate: parseToOrigin(option.rate, option.rate_decimals),
        rate_decimals: Number(option.rate_decimals)
    };
    const res = await actor!.setFee(fee, [newNonce]);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(updatedNonce, newNonce);
});
When(/^I update token "([^"]*)"'s feeTo as "([^"]*)" with owner "([^"]*)" with wrong nonce, will fail, the nonce will not change$/, async function (token, feeTo, owner) {
    const actor = createDFTActor(token, owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const feeToPrincipal = identityFactory.getPrincipal(feeTo)!.toText();
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const wrongNonce = crtNonce + 2n;
    const res = await actor!.setFeeTo(feeToPrincipal, [wrongNonce]);
    assert.isTrue("Err" in res);
    const updatedNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(updatedNonce, crtNonce);
});
When(/^I update token "([^"]*)"'s feeTo as "([^"]*)" with owner "([^"]*)" with out nonce, the nonce should increase (\d+)$/, async function (token, feeTo, owner, increase) {
    const actor = createDFTActor(token, owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const feeToPrincipal = identityFactory.getPrincipal(feeTo)!.toText();
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const res = await actor!.setFeeTo(feeToPrincipal, []);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(updatedNonce, newNonce);
});
When(/^I update token "([^"]*)"'s feeTo as "([^"]*)" with owner "([^"]*)" with correct nonce, the nonce should increase (\d+)$/, async function (token, feeTo, owner, increase) {
    const actor = createDFTActor(token, owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const feeToPrincipal = identityFactory.getPrincipal(feeTo)!.toText();
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const res = await actor!.setFeeTo(feeToPrincipal, [newNonce]);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(updatedNonce, newNonce);
});
When(/^"([^"]*)" burn (\d+) "([^"]*)" token with wrong nonce, will fail, the nonce will not change$/, async function (burner, amount, token) {
    const actor = createDFTBurnableActor(burner);
    const burnerPrincipal = identityFactory.getPrincipal(burner)!;
    const crtNonce = await actor!.nonceOf(burnerPrincipal);
    const wrongNonce = crtNonce + 2n;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.burn([], amountBN, [wrongNonce]);
    assert.isTrue("Err" in res);
    const updatedNonce = await actor!.nonceOf(burnerPrincipal);
    assert.equal(updatedNonce, crtNonce);
});
When(/^"([^"]*)" burn (\d+) "([^"]*)" token with out nonce, the nonce should increase (\d+)$/, async function (burner, amount, token, increase) {
    const actor = createDFTBurnableActor(burner);
    const burnerPrincipal = identityFactory.getPrincipal(burner)!;
    const crtNonce = await actor!.nonceOf(burnerPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.burn([], amountBN, []);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(burnerPrincipal);
    assert.equal(updatedNonce, newNonce);
});

When(/^"([^"]*)" burn (\d+) "([^"]*)" token with correct nonce, the nonce should increase (\d+)$/, async function (burner, amount, token, increase) {
    const actor = createDFTBurnableActor(burner);
    const burnerPrincipal = identityFactory.getPrincipal(burner)!;
    const crtNonce = await actor!.nonceOf(burnerPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.burn([], amountBN, [newNonce]);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(burnerPrincipal);
    assert.equal(updatedNonce, newNonce);
});

Then(/^"([^"]*)" burn "([^"]*)" from "([^"]*)" "([^"]*)" token with wrong nonce, will fail, the nonce will not change$/, async function (burner, amount, owner, token) {
    const actor = createDFTBurnableActor(burner);
    const burnerPrincipal = identityFactory.getPrincipal(burner)!;
    const ownerPrincipal = identityFactory.getPrincipal(owner)!.toText();
    const crtNonce = await actor!.nonceOf(burnerPrincipal);
    const wrongNonce = crtNonce + 2n;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.burnFrom([], ownerPrincipal, amountBN, [wrongNonce]);
    assert.isTrue("Err" in res);
    const updatedNonce = await actor!.nonceOf(burnerPrincipal);
    assert.equal(updatedNonce, crtNonce);
});

When(/^"([^"]*)" burn "([^"]*)" from "([^"]*)" "([^"]*)" token with out nonce, the nonce should increase (\d+)$/, async function (burner, amount, owner, token, increase) {
    const actor = createDFTBurnableActor(burner);
    const burnerPrincipal = identityFactory.getPrincipal(burner)!;
    const ownerPrincipal = identityFactory.getPrincipal(owner)!.toText();
    const crtNonce = await actor!.nonceOf(burnerPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.burnFrom([], ownerPrincipal, amountBN, []);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(burnerPrincipal);
    assert.equal(updatedNonce, newNonce);
});

When(/^"([^"]*)" burn "([^"]*)" from "([^"]*)" "([^"]*)" token with correct nonce, the nonce should increase (\d+)$/, async function (burner, amount, owner, token, increase) {
    const actor = createDFTBurnableActor(burner);
    const burnerPrincipal = identityFactory.getPrincipal(burner)!;
    const ownerPrincipal = identityFactory.getPrincipal(owner)!.toText();
    const crtNonce = await actor!.nonceOf(burnerPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.burnFrom([], ownerPrincipal, amountBN, [newNonce]);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(burnerPrincipal);
    assert.equal(updatedNonce, newNonce);
});
When(/^"([^"]*)" mint (\d+) "([^"]*)" for "([^"]*)" token with wrong nonce, will fail, the nonce will not change$/, async function (owner, amount, token, to) {
    const actor = createDFTMintableActor(owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const wrongNonce = crtNonce + 2n;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.mint(toPrincipal, amountBN, [wrongNonce]);
    assert.isTrue("Err" in res);
    const updatedNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(updatedNonce, crtNonce);
});
When(/^"([^"]*)" mint (\d+) "([^"]*)" for "([^"]*)" token with out nonce, the nonce should increase (\d+)$/, async function (owner, amount, token, to, increase) {
    const actor = createDFTMintableActor(owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.mint(toPrincipal, amountBN, []);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(updatedNonce, newNonce);
});
When(/^"([^"]*)" mint (\d+) "([^"]*)" for "([^"]*)" token with correct nonce, the nonce should increase (\d+)$/, async function (owner, amount, token, to, increase) {
    const actor = createDFTMintableActor(owner);
    const ownerPrincipal = identityFactory.getPrincipal(owner)!;
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const crtNonce = await actor!.nonceOf(ownerPrincipal);
    const increaseBN = BigInt(increase);
    const newNonce = crtNonce + increaseBN;
    const decimals = await actor!.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const res = await actor!.mint(toPrincipal, amountBN, [newNonce]);
    assert.isTrue("Ok" in res);
    const updatedNonce = await actor!.nonceOf(ownerPrincipal);
    assert.equal(updatedNonce, newNonce);
});
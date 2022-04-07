import {When, Then} from "@cucumber/cucumber";
import {parseRawTableToJsonArray} from "~/utils/convert";
import {assert, expect} from "chai";
import {createDFTActor, fileToByteArray} from "./utils";
import {Fee} from "~/declarations/dft_basic/dft_basic.did";
import {parseToOrigin} from "~/utils/uint";
import {identityFactory} from "~/utils/identity";
import {existsSync, readFileSync} from "fs";
import path from "path";

When(/^I update token "([^"]*)"'s description with not owner "([^"]*)", will failed$/, async function (token, user, {rawTable}) {
    let actor = createDFTActor(token, user);
    const optionArray = parseRawTableToJsonArray(rawTable);
    let desc: Array<[string, string]> = [];
    //convert optionArray to desc
    for (let i = 0; i < optionArray.length; i++) {
        desc.push([optionArray[i].key, optionArray[i].value]);
    }
    try {
        let res = await actor!.setDesc(desc,[])
        expect.fail(`should not set success, but set success with ${res}`);
    } catch (e) {
        // should be here
    }
});
When(/^I update token "([^"]*)"'s description with owner "([^"]*)", will success$/, async function (token, user, {rawTable}) {
    const actor = createDFTActor(token, user);
    const optionArray = parseRawTableToJsonArray(rawTable);
    let desc: Array<[string, string]> = [];
    //convert optionArray to desc
    for (let i = 0; i < optionArray.length; i++) {
        desc.push([optionArray[i].key, optionArray[i].value]);
    }
    const res = await actor!.setDesc(desc,[]);
    assert.isTrue("Ok" in res);
});

Then(/^Get token "([^"]*)"'s description will not contain "([^"]*)" and "([^"]*)" by "([^"]*)"$/, async function (token, key1, key2, user) {
    const actor = createDFTActor(token, user);

    const res = await actor!.desc();
    assert.isTrue(!res.includes(key1));
    assert.isTrue(!res.includes(key2));
});

Then(/^Get token "([^"]*)"'s description by "([^"]*)",will include blow fields and values$/, async function (token, user, {rawTable}) {
    const actor = createDFTActor(token, user);
    const optionArray = parseRawTableToJsonArray(rawTable);
    let desc: Array<[string, string]> = [];
    //convert optionArray to desc
    for (let i = 0; i < optionArray.length; i++) {
        desc.push([optionArray[i].key, optionArray[i].value]);
    }
    const res = await actor!.desc();

    // check res include desc
    for (let i = 0; i < desc.length; i++) {
        for (let j = 0; j < res.length; j++) {
            if (res[j].includes(desc[i][0])) {
                assert.isTrue(res[j].includes(desc[i][1]));
            }
        }
    }
});

When(/^I update token "([^"]*)"'s logo "([^"]*)" with owner "([^"]*)", will success$/, async function (token, logoName, user) {
    const logoData = fileToByteArray(`./scripts/assets/${logoName}`);
    const logoParam: [number[]] = [Array.from(logoData)];
    const actor = createDFTActor(token, user);
    const res = await actor!.setLogo(logoParam,[]);
    assert.isTrue("Ok" in res, `set logo failed with ${JSON.stringify(res)}`);
});

When(/^I update token "([^"]*)"'s logo with invalid image data with owner "([^"]*)", will failed$/, async function (token, user) {
    const logoData: [number[]] = [[1, 2, 3, 4]];
    const actor = createDFTActor(token, user);
    try {
        const res = await actor!.setLogo(logoData,[]);
        expect.fail(`should not set success, but set success with ${res}`);
    } catch (e) {
        // should be here
    }
});
When(/^I update token "([^"]*)"'s logo with not owner "([^"]*)", will failed$/, async function (token, user) {
    const logoData: any = [[1, 2, 3, 4]];
    const actor = createDFTActor(token, user);
    try {
        const res = await actor!.setLogo(logoData,[]);
        expect.fail(`should not set success, but set success with ${res}`);
    } catch (e) {
        // should be here
    }
});

When(/^I update token "([^"]*)"'s fee with owner "([^"]*)", will success$/, async function (token, owner, {rawTable}) {
    const actor = createDFTActor(token, owner);
    const optionArray = parseRawTableToJsonArray(rawTable);
    const option = optionArray[0];
    const decimals = await actor!.decimals();
    // convert optionArray to TokenFee
    const fee: Fee = {
        minimum: parseToOrigin(option.minimum, decimals),
        rate: parseToOrigin(option.rate, option.rate_decimals),
        rate_decimals: Number(option.rate_decimals)
    };
    const res = await actor!.setFee(fee, []);
    assert.isTrue("Ok" in res);
});
Then(/^Get token "([^"]*)"'s fee by "([^"]*)",will include blow fields and value$/, async function (token, user, {rawTable}) {
    const actor = createDFTActor(token, user);
    const optionArray = parseRawTableToJsonArray(rawTable);
    const option = optionArray[0];
    const decimals = await actor!.decimals();
    const fee = await actor!.fee();
    const feeValid: Fee = {
        minimum: parseToOrigin(option.minimum, decimals),
        rate: parseToOrigin(option.rate, option.rate_decimals),
        rate_decimals: Number(option.rate_decimals)
    };
    // check fee is same with feeValid
    assert.equal(fee.minimum, feeValid.minimum);
    assert.equal(fee.rate, feeValid.rate);
    assert.equal(fee.rate_decimals, feeValid.rate_decimals);
});
When(/^I update token "([^"]*)"'s fee with not owner "([^"]*)", will failed$/, async function (token, owner, {rawTable}) {
    const actor = createDFTActor(token, owner);
    const optionArray = parseRawTableToJsonArray(rawTable);
    const option = optionArray[0];
    const decimals = await actor!.decimals();
    // convert optionArray to TokenFee
    const fee: Fee = {
        minimum: parseToOrigin(option.minimum, decimals),
        rate: parseToOrigin(option.rate, option.rate_decimals),
        rate_decimals: Number(option.rate_decimals)
    };
    try {
        const res = await actor!.setFee(fee, []);
        expect.fail(`should not set success, but set success with ${res}`);
    } catch (e) {
        // should be here
    }
});
When(/^I update token "([^"]*)"'s feeTo as "([^"]*)" with owner "([^"]*)", will success$/, async function (token, feeTo, owner) {
    const actor = createDFTActor(token, owner);
    const feeToPrincipal = identityFactory.getPrincipal(feeTo)!.toText();
    const res = await actor!.setFeeTo(feeToPrincipal, []);
    assert.isTrue("Ok" in res);
});
Then(/^Get token "([^"]*)"'s feeTo by "([^"]*)", should be "([^"]*)"$/, async function (token, owner, feeTo) {
    const actor = createDFTActor(token, owner);
    const feeToPrincipal = identityFactory.getPrincipal(feeTo)!.toText();
    const res = await actor!.tokenInfo();
    const feeToRes: any = res.feeTo;
    assert.equal(feeToRes.Principal.toText(), feeToPrincipal);
});
When(/^I update token "([^"]*)"'s feeTo as "([^"]*)" with not owner "([^"]*)", will failed$/, async function (token, feeTo, owner) {
    const actor = createDFTActor(token, owner);
    const feeToPrincipal = identityFactory.getPrincipal(feeTo)!.toText();
    try {
        const res = await actor!.setFeeTo(feeToPrincipal, []);
        expect.fail(`should not set success, but set success with ${res}`);
    } catch (e) {
        // should be here
    }
});


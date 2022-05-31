import "./setup";
import { When, Then, DataTable } from "@cucumber/cucumber";
import { assert, expect } from "chai";
import { createDFTActor, fileToByteArray } from "./utils";
import { TokenFee } from "../../src/scripts/declarations/dft_basic/dft_basic.did";
import { unit, identity, canister } from "@deland-labs/ic-dev-kit";


When(/^I update token "([^"]*)"'s description with not owner "([^"]*)", will failed$/, async function (token, user, dataTable: DataTable) {
    let actor = createDFTActor(token, user);
    const optionArray = dataTable.hashes();
    let desc: Array<[string, string]> = [];
    //convert optionArray to desc
    for (let i = 0; i < optionArray.length; i++) {
        desc.push([optionArray[i].key, optionArray[i].value]);
    }
    try {
        let res = await actor!.setDesc(desc)
        expect.fail(`should not set success, but set success with ${res}`);
    } catch (e) {
        // should be here
    }
});
When(/^I update token "([^"]*)"'s description with owner "([^"]*)", will success$/, async function (token, user, dataTable: DataTable) {
    const actor = createDFTActor(token, user);
    const optionArray = dataTable.hashes();
    let desc: Array<[string, string]> = [];
    //convert optionArray to desc
    for (let i = 0; i < optionArray.length; i++) {
        desc.push([optionArray[i].key, optionArray[i].value]);
    }
    const res = await actor!.setDesc(desc);
    assert.isTrue("Ok" in res);
});

Then(/^Get token "([^"]*)"'s description will not contain "([^"]*)" and "([^"]*)" by "([^"]*)"$/, async function (token, key1, key2, user) {
    const actor = createDFTActor(token, user);

    const res = await actor!.desc();
    assert.isTrue(!res.includes(key1));
    assert.isTrue(!res.includes(key2));
});

Then(/^Get token "([^"]*)"'s description by "([^"]*)",will include blow fields and values$/, async function (token, user, dataTable: DataTable) {
    const actor = createDFTActor(token, user);
    const optionArray = dataTable.hashes();
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
    const res = await actor!.setLogo(logoParam);
    assert.isTrue("Ok" in res, `set logo failed with ${JSON.stringify(res)}`);
});

When(/^I update token "([^"]*)"'s logo with invalid image data with owner "([^"]*)", will failed$/, async function (token, user) {
    const logoData: [number[]] = [[1, 2, 3, 4]];
    const actor = createDFTActor(token, user);
    try {
        const res = await actor!.setLogo(logoData);
        expect.fail(`should not set success, but set success with ${res}`);
    } catch (e) {
        // should be here
    }
});
When(/^I update token "([^"]*)"'s logo with not owner "([^"]*)", will failed$/, async function (token, user) {
    const logoData: any = [[1, 2, 3, 4]];
    const actor = createDFTActor(token, user);
    try {
        const res = await actor!.setLogo(logoData);
        expect.fail(`should not set success, but set success with ${res}`);
    } catch (e) {
        // should be here
    }
});

When(/^I update token "([^"]*)"'s fee with owner "([^"]*)", will success$/, async function (token, owner, dataTable: DataTable) {
    const actor = createDFTActor(token, owner);
    const optionArray = dataTable.hashes();
    const option = optionArray[0];
    const decimals = await actor!.decimals();
    // convert optionArray to InnerTokenFee
    const fee: TokenFee = {
        minimum: unit.parseToOrigin(option.minimum, decimals),
        rate: Number(unit.parseToOrigin(option.rate, option.rate_decimals)),
        rateDecimals: Number(option.rate_decimals)
    };
    const res = await actor!.setFee(fee, []);
    assert.isTrue("Ok" in res);
});
Then(/^Get token "([^"]*)"'s fee by "([^"]*)",will include blow fields and value$/, async function (token, user, dataTable: DataTable) {
    const actor = createDFTActor(token, user);
    const optionArray = dataTable.hashes();
    const option = optionArray[0];
    const decimals = await actor!.decimals();
    const fee = await actor!.fee();
    const feeValid: TokenFee = {
        minimum: unit.parseToOrigin(option.minimum, decimals),
        rate: Number(unit.parseToOrigin(option.rate, option.rate_decimals)),
        rateDecimals: Number(option.rate_decimals)
    };
    // check fee is same with feeValid
    assert.equal(fee.minimum, feeValid.minimum);
    assert.equal(fee.rate, feeValid.rate);
    assert.equal(fee.rateDecimals, feeValid.rateDecimals);
});
When(/^I update token "([^"]*)"'s fee with not owner "([^"]*)", will failed$/, async function (token, owner, dataTable: DataTable) {
    const actor = createDFTActor(token, owner);
    const optionArray = dataTable.hashes();
    const option = optionArray[0];
    const decimals = await actor!.decimals();
    // convert optionArray to InnerTokenFee
    const fee: TokenFee = {
        minimum: unit.parseToOrigin(option.minimum, decimals),
        rate: Number(unit.parseToOrigin(option.rate, option.rate_decimals)),
        rateDecimals: Number(option.rate_decimals)
    };
    try {
        const res = await actor!.setFee(fee, []);
        expect.fail(`should not set success, but set success with ${res}`);
    } catch (e) {
        // should be here
    }
});

When(/^I update token "([^"]*)"'s fee with owner "([^"]*)" twice, the second will fail$/, async function (token, owner, dataTable: DataTable) {
    const actor = createDFTActor(token, owner);
    const optionArray = dataTable.hashes();
    const option = optionArray[0];
    const decimals = await actor!.decimals();
    // convert optionArray to InnerTokenFee
    const fee: TokenFee = {
        minimum: unit.parseToOrigin(option.minimum, decimals),
        rate: Number(unit.parseToOrigin(option.rate, option.rate_decimals)),
        rateDecimals: Number(option.rate_decimals)
    };

    //  set created_at as nanos timestamp
    const created_at = BigInt(new Date().getTime()) * 1000000n;
    const res = await actor!.setFee(fee, [created_at]);
    assert.isTrue("Ok" in res);

    const res2 = await actor!.setFee(fee, [created_at]);
    assert.isTrue("Err" in res2);
});
When(/^I update token "([^"]*)"'s feeTo as "([^"]*)" with owner "([^"]*)", will success$/, async function (token, feeTo, owner) {
    const actor = createDFTActor(token, owner);
    const feeToPrincipal = identity.identityFactory.getPrincipal(feeTo)!.toText();
    const res = await actor!.setFeeTo(feeToPrincipal, []);
    assert.isTrue("Ok" in res);
});
Then(/^Get token "([^"]*)"'s feeTo by "([^"]*)", should be "([^"]*)"$/, async function (token, owner, feeTo) {
    const actor = createDFTActor(token, owner);
    const feeToAccountId = identity.identityFactory.getAccountIdHex(feeTo);
    const res = await actor!.tokenInfo();
    assert.equal(res.feeTo, feeToAccountId);
});
When(/^I update token "([^"]*)"'s feeTo as "([^"]*)" with not owner "([^"]*)", will failed$/, async function (token, feeTo, owner) {
    const actor = createDFTActor(token, owner);
    const feeToPrincipal = identity.identityFactory.getPrincipal(feeTo)!.toText();
    try {
        const res = await actor!.setFeeTo(feeToPrincipal, []);
        expect.fail(`should not set success, but set success with ${res}`);
    } catch (e) {
        // should be here
    }
});
When(/^I update token "([^"]*)"'s feeTo as "([^"]*)" with owner "([^"]*)" twice, the second will fail$/, async function (token, feeTo, owner) {
    const actor = createDFTActor(token, owner);
    const feeToPrincipal = identity.identityFactory.getPrincipal(feeTo)!.toText();
    //  set created_at as nanos timestamp
    const created_at = BigInt(new Date().getTime()) * 1000000n;
    const res = await actor!.setFeeTo(feeToPrincipal, [created_at]);
    assert.isTrue("Ok" in res);

    const res2 = await actor!.setFeeTo(feeToPrincipal, [created_at]);
    assert.isTrue("Err" in res2);
});
When(/^I update token "([^"]*)"'s owner to "([^"]*)" with owner "([^"]*)", will success$/, async function (token, newOwner, owner) {
    const actor = createDFTActor(token, owner);
    const newOwnerPrincipal = identity.identityFactory.getPrincipal(newOwner)!;
    const res = await actor!.setOwner(newOwnerPrincipal, []);
    assert.isTrue("Ok" in res);
});

Then(/^Get token "([^"]*)"'s owner by "([^"]*)", should be "([^"]*)"$/, async function (token, caller, owner) {
    const actor = createDFTActor(token, caller);
    const ownerPrincipal = identity.identityFactory.getPrincipal(owner)!;
    const ownerRes = await actor!.owner();
    assert.equal(ownerRes.toText(), ownerPrincipal.toText());
});
When(/^I update token "([^"]*)"'s to "([^"]*)" owner with not owner "([^"]*)", will failed$/, async function (token, newOwner, notOwner) {
    const actor = createDFTActor(token, notOwner);
    const newOwnerPrincipal = identity.identityFactory.getPrincipal(newOwner)!;
    try {
        const res = await actor!.setOwner(newOwnerPrincipal, []);
        expect.fail(`should not set success, but set success with ${res}`);
    } catch (e) {
        // should be here
    }
});

When(/^I update token "([^"]*)"'s fee with owner "([^"]*)" with passed "(\d+)" days, will failed$/, async function (token, owner, passedDays, dataTable: DataTable) {
    const actor = createDFTActor(token, owner);
    const optionArray = dataTable.hashes();
    const option = optionArray[0];
    const decimals = await actor!.decimals();
    const nowNanos = BigInt(new Date().getTime()) * 1000000n;
    const passedNanos = nowNanos - BigInt(passedDays) * 24n * 60n * 60n * 1000000000n;
    // convert optionArray to InnerTokenFee
    const newFee: TokenFee = {
        minimum: unit.parseToOrigin(option.minimum, decimals),
        rate: Number(unit.parseToOrigin(option.rate, option.rate_decimals)),
        rateDecimals: Number(option.rate_decimals)
    };
    const res = await actor!.setFee(newFee, [passedNanos]);
    assert.isTrue("Err" in res);
});
When(/^I update token "([^"]*)"'s feeTo as "([^"]*)" with owner "([^"]*)" with passed "([^"]*)" days, will failed$/, async function (token, newFeeTo, owner, passedDays) {
    const actor = createDFTActor(token, owner);
    const newFeeToPrincipal = identity.identityFactory.getPrincipal(newFeeTo)!.toText();
    const nowNanos = BigInt(new Date().getTime()) * 1000000n;
    const passedNanos = nowNanos - BigInt(passedDays) * 24n * 60n * 60n * 1000000000n;
    const res = await actor!.setFeeTo(newFeeToPrincipal, [passedNanos]);
    assert.isTrue("Err" in res);
});
When(/^I update token "([^"]*)"'s owner to "([^"]*)" with owner "([^"]*)" with passed "([^"]*)" days, will failed$/, async function (token, newOwner, owner, passedDays) {
    const actor = createDFTActor(token, owner);
    const newOwnerPrincipal = identity.identityFactory.getPrincipal(newOwner)!;
    const nowNanos = BigInt(new Date().getTime()) * 1000000n;
    const passedNanos = nowNanos - BigInt(passedDays) * 24n * 60n * 60n * 1000000000n;
    const res = await actor!.setOwner(newOwnerPrincipal, [passedNanos]);
    assert.isTrue("Err" in res);
});
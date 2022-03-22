import {When, Then} from "@cucumber/cucumber";
import {
    createDFTBasic2Actor,
    createDFTBasicActor,
    createDFTBurnableActor,
    createDFTMintableActor
} from "~/declarations";
import {parseRawTableToJsonArray} from "~/utils/convert";
import {assert, expect} from "chai";
import logger from "node-color-log";
import {cat} from "shelljs";


When(/^I update token "([^"]*)"'s description with not owner\("([^"]*)"\), will failed$/, async function (token, user, {rawTable}) {
    let actor = createDFTBasicActor(user);
    const optionArray = parseRawTableToJsonArray(rawTable);
    let desc: Array<[string, string]> = [];
    //convert optionArray to desc
    for (let i = 0; i < optionArray.length; i++) {
        desc.push([optionArray[i].key, optionArray[i].value]);
    }
    switch (token) {
        case "dft_basic":
            actor = createDFTBasicActor(user);
            break;
        case "dft_basic_2":
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
    try {
        let res = await actor.setDesc(desc)
        expect.fail(`should not set success, but set success with ${res}`);
    } catch (e) {
        // should be here
    }
});
When(/^I update token "([^"]*)"'s description with owner\("([^"]*)"\), will success$/, async function (token, user, {rawTable}) {
    let actor = createDFTBasicActor(user);
    const optionArray = parseRawTableToJsonArray(rawTable);
    let desc: Array<[string, string]> = [];
    //convert optionArray to desc
    for (let i = 0; i < optionArray.length; i++) {
        desc.push([optionArray[i].key, optionArray[i].value]);
    }
    switch (token) {
        case "dft_basic":
            actor = createDFTBasicActor(user);
            break;
        case "dft_basic_2":
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

    const res = await actor.setDesc(desc);
    assert.isTrue("Ok" in res);
});

Then(/^Get token "([^"]*)"'s description will not contain "([^"]*)" and "([^"]*)" by "([^"]*)"$/, async function (token, key1, key2, user) {
    let actor = createDFTBasicActor(user);
    switch (token) {
        case "dft_basic":
            actor = createDFTBasicActor(user);
            break;
        case "dft_basic_2":
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

    const res = await actor.desc();
    assert.isTrue(!res.includes(key1));
    assert.isTrue(!res.includes(key2));
});

Then(/^Get token "([^"]*)"'s description by "([^"]*)",will include blow fields and values$/, async function (token, user, {rawTable}) {
    let actor = createDFTBasicActor(user);
    const optionArray = parseRawTableToJsonArray(rawTable);
    let desc: Array<[string, string]> = [];
    //convert optionArray to desc
    for (let i = 0; i < optionArray.length; i++) {
        desc.push([optionArray[i].key, optionArray[i].value]);
    }
    switch (token) {
        case "dft_basic":
            actor = createDFTBasicActor(user);
            break;
        case "dft_basic_2":
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
    const res = await actor.desc();

    // check res include desc
    for (let i = 0; i < desc.length; i++) {
        for (let j = 0; j < res.length; j++) {
            if (res[j].includes(desc[i][0])) {
                assert.isTrue(res[j].includes(desc[i][1]));
            }
        }
    }
});
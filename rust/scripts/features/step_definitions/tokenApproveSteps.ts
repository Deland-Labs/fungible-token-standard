import {Given} from "@cucumber/cucumber";
import {parseRawTableToJsonArray} from "~/utils/convert";
import {
    createDFTBasic2Actor,
    createDFTBasicActor,
    createDFTBurnableActor,
    createDFTMintableActor
} from "~/declarations";
import {identityFactory} from "~/utils/identity";
import {parseToOrigin} from "~/utils/uint";
import {assert} from "chai";

Given(/^approve tokens from owner to spender in table$/, async function ({rawTable}) {
    const optionArray = parseRawTableToJsonArray(rawTable);
    for (let i = 0; i < optionArray.length; i++) {
        const option = optionArray[i];
        let dftActor = createDFTBasicActor(option.owner);
        const owner = identityFactory.getPrincipal(option.owner)!.toText();
        switch (option.token) {
            case "dft_basic":
                dftActor = createDFTBasicActor(option.owner);
                break;
            case "dft_basic2":
                dftActor = createDFTBasic2Actor(option.owner);
                break;
            case "dft_burnable":
                dftActor = createDFTBurnableActor(option.owner);
                break;
            case "dft_mintable":
                dftActor = createDFTMintableActor(option.owner);
                break;
            default:
                break;
        }
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
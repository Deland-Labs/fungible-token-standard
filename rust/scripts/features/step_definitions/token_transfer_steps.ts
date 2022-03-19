import {Given, Then, When} from "@cucumber/cucumber";
import {assert, expect} from "chai";
import logger from "node-color-log";
import {CanisterReinstallOptions, DFTInitOptions, reinstall_all} from "../../src/tasks";
import {parseToOrigin} from "~/utils/uint";
import {
    createDFTBasic2Actor,
    createDFTBasicActor,
    createDFTBurnableActor,
    createDFTMintableActor
} from "~/declarations";
import {parseRawTableToJsonArray} from "~/utils/convert";
import {identityFactory} from "~/utils/identity";

Given(/^Reinstall dft canisters$/, async ({rawTable}) => {
    let optionArray: Array<any> = parseRawTableToJsonArray(rawTable);
    // dft basic option
    let dftBasicOption = optionArray.find(o => o.key === "dft_basic");
    let dftBasicInitOptions = parseToDFTInitOptions(dftBasicOption);
    // dft basic 2 option
    let dftBasic2Option = optionArray.find(o => o.key === "dft_basic2");
    let dftBasic2InitOptions = parseToDFTInitOptions(dftBasic2Option);
    // dft burn able option
    let dftBurnAbleOption = optionArray.find(o => o.key === "dft_burnable");
    let dftBurnAbleInitOptions = parseToDFTInitOptions(dftBurnAbleOption);
    // dft mint able option
    let dftMintAbleOption = optionArray.find(o => o.key === "dft_mintable");
    let dftMintAbleInitOptions = parseToDFTInitOptions(dftMintAbleOption);

    let reinstallOptions: CanisterReinstallOptions = {
            build: false,
            init: false,
            one_by_one: false,
            canisters: {
                dft_basic: dftBasicInitOptions ? {
                    reinstall: true,
                    initOptions: dftBasicInitOptions
                } : undefined,
                dft_basic2: dftBasic2InitOptions ? {
                    reinstall: true,
                    initOptions: dftBasic2InitOptions
                } : undefined,
                dft_burnable: dftBurnAbleInitOptions ? {
                    reinstall: true,
                    initOptions: dftBurnAbleInitOptions
                } : undefined,
                dft_mintable: dftMintAbleInitOptions ? {
                    reinstall: true,
                    initOptions: dftMintAbleInitOptions
                } : undefined,
                dft_receiver: {reinstall: true},
                dft_tx_storage: {reinstall: true},
            }
        }
    ;
    await reinstall_all(reinstallOptions);
    logger.debug(`option array: ${JSON.stringify(optionArray)}`);
});

Given(/^transfer tokens from "([^"]*)" to these users$/, async function (user, args) {
    const dftBasic = createDFTBasicActor(user);
    const dftBasic2 = createDFTBasic2Actor(user);
    const dftBurnAble = createDFTBurnableActor(user);
    const dftMintAble = createDFTMintableActor(user);

    const dftActors = [dftBasic, dftBasic2, dftBurnAble, dftMintAble];

    const optionArray = parseRawTableToJsonArray(args.rawTable);
    for (let i = 0; i < optionArray.length; i++) {
        const option = optionArray[i];
        for (let j = 0; j < dftActors.length; j++) {
            const dftActor = dftActors[j];
            if (dftActor && option) {
                const decimals = await dftActor.decimals();
                const to = identityFactory.getPrincipal(option.user)!.toText();
                const amountBN = parseToOrigin(option.amount, decimals);
                const res = await dftActor.transfer([], to, amountBN, []);
                assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);
                assert.equal(await dftActor.balanceOf(to), amountBN);
            }
        }
    }
});

When(/^Transfer from (.*) to (.*),(.*) (.*)$/, async function (userA, userB, diff, token) {
    logger.debug(`Transfer from ${userA} to ${userB},${diff} ${token}`);
    const userAPrincipal = identityFactory.getPrincipal(userA)!;

    const userBPrincipal = identityFactory.getPrincipal(userB)!;
    switch (token) {
        case "dft_basic":
            const actor = createDFTBasicActor(userA);
            const decimals = await actor.decimals();
            const amountBN = parseToOrigin(diff, decimals);
            const res = await actor.transfer([], userBPrincipal.toText(), amountBN, []);
            assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);

            break;
        case "dft_basic2":
            const actor2 = createDFTBasic2Actor(userA);
            const decimals2 = await actor2.decimals();
            const amountBN2 = parseToOrigin(diff, decimals2);
            const res2 = await actor2.transfer([], userBPrincipal.toText(), amountBN2, []);
            assert.isTrue('Ok' in res2, `transfer failed: ${JSON.stringify(res2)}`);
            break;
        case "dft_burnable":
            const actor3 = createDFTBurnableActor(userA);
            const decimals3 = await actor3.decimals();
            const amountBN3 = parseToOrigin(diff, decimals3);
            const res3 = await actor3.transfer([], userBPrincipal.toText(), amountBN3, []);
            assert.isTrue('Ok' in res3, `transfer failed: ${JSON.stringify(res3)}`);
            break;
        case "dft_mintable":
            const actor4 = createDFTMintableActor(userA);
            const decimals4 = await actor4.decimals();
            const amountBN4 = parseToOrigin(diff, decimals4);
            const res4 = await actor4.transfer([], userBPrincipal.toText(), amountBN4, []);
            assert.isTrue('Ok' in res4, `transfer failed: ${JSON.stringify(res4)}`);
            break;
        default:
            break;
    }
});

Then(/^Check the (.*) balance of (.*) should be (.*)$/, async function (token, user, balance) {
    const userPrincipal = identityFactory.getPrincipal(user)!;
    switch (token) {
        case "dft_basic":
            const actor = createDFTBasicActor(user);
            const decimals = await actor.decimals();
            const balanceBN = parseToOrigin(balance, decimals);
            assert.equal(balanceBN, await actor.balanceOf(userPrincipal.toText()));
            break;
        case "dft_basic2":
            const actor2 = createDFTBasic2Actor(user);
            const decimals2 = await actor2.decimals();
            const balanceBN2 = parseToOrigin(balance, decimals2);
            assert.equal(balanceBN2, await actor2.balanceOf(userPrincipal.toText()));
            break;
        case "dft_burnable":
            const actor3 = createDFTBurnableActor(user);
            const decimals3 = await actor3.decimals();
            const balanceBN3 = parseToOrigin(balance, decimals3);
            assert.equal(balanceBN3, await actor3.balanceOf(userPrincipal.toText()));
            break;
        case "dft_mintable":
            const actor4 = createDFTMintableActor(user);
            const decimals4 = await actor4.decimals();
            const balanceBN4 = parseToOrigin(balance, decimals4);
            assert.equal(balanceBN4, await actor4.balanceOf(userPrincipal.toText()));
            break;
        default:
            break;
    }
});

Then(/^Check that the transfer fees of (.*) by (.*) charged are correct$/, function (token, fee) {

});

const parseToDFTInitOptions = (option: any): DFTInitOptions | undefined => {
    logger.debug(`option is ${JSON.stringify(option)}`);
    const decimals = parseInt(option.decimals);
    const feeDecimals = parseInt(option.rate_decimals);
    // if option is undefined, return undefined
    if (!option) return undefined;
    return {
        name: String(option.name),
        symbol: String(option.symbol),
        decimals: BigInt(decimals),
        totalSupply: parseToOrigin(option.total_supply, decimals),
        fee: {
            minimum: Number(parseToOrigin(option.fee_minimum, decimals)),
            rate: Number(option.fee_rate != 0 ? parseToOrigin(option.fee_rate, feeDecimals) : 0n),
            rate_decimals: feeDecimals,
        },
        desc: [],
        owner: identityFactory.getPrincipal(option.owner)!.toText(),
    };
}

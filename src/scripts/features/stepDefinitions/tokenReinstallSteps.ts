import "./setup";
import { DataTable, Given } from "@cucumber/cucumber";
import { assert } from "chai";
import logger from "node-color-log";
import { CanisterReinstallOptions, DFTInitOptions, reinstall_all } from "../../src/tasks";
import {
    createDFTWithAllFeatures,
    createDFTBasicActor,
    createDFTBurnableActor,
    createDFTMintableActor
} from "../../src/scripts/declarations";
import { createDFTActor } from "./utils";
import { unit, identity } from "@deland-labs/ic-dev-kit";


Given(/^Reinstall dft canisters$/, async (dataTable: DataTable) => {
    let optionArray: Array<any> = dataTable.hashes();
    // dft basic option
    let dftBasicOption = optionArray.find(o => o.key === "dft_basic");
    let dftBasicInitOptions = parseToDFTInitOptions(dftBasicOption);
    // dft basic 2 option
    let dftBasic2Option = optionArray.find(o => o.key === "dft_all_features");
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
            dft_all_features: dftBasic2InitOptions ? {
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
            dft_receiver: { reinstall: true },
            dft_tx_storage: { reinstall: false }
        }
    }
        ;
    await reinstall_all(reinstallOptions);
    logger.debug(`option array: ${JSON.stringify(optionArray)}`);
});

Given(/^transfer tokens from "([^"]*)" to these users$/, async function (user, dataTable: DataTable) {
    const dftBasic = createDFTBasicActor(user);
    const dftBasic2 = createDFTWithAllFeatures(user);
    const dftBurnAble = createDFTBurnableActor(user);
    const dftMintAble = createDFTMintableActor(user);

    const dftActors = [dftBasic, dftBasic2, dftBurnAble, dftMintAble];

    const optionArray = dataTable.hashes();
    for (let i = 0; i < optionArray.length; i++) {
        const option = optionArray[i];
        for (let j = 0; j < dftActors.length; j++) {
            const dftActor = dftActors[j];
            if (dftActor && option) {
                const decimals = await dftActor.decimals();
                const to = identity.identityFactory.getPrincipal(option.user)!.toText();
                const amountBN = unit.parseToOrigin(option.amount, decimals);
                const res = await dftActor.transfer([], to, amountBN, []);
                assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);
                assert.equal(await dftActor.balanceOf(to), amountBN);
            }
        }
    }
});

Given(/^transfer token from "([^"]*)" to these users$/, async function (user, dataTable: DataTable) {
    const optionArray = dataTable.hashes();
    for (let i = 0; i < optionArray.length; i++) {
        const option = optionArray[i];
        let dftActor = createDFTActor(option.token, user);
        const decimals = await dftActor!.decimals();
        const to = identity.identityFactory.getPrincipal(option.user)!.toText();
        const amountBN = unit.parseToOrigin(option.amount, decimals);
        const res = await dftActor!.transfer([], to, amountBN, []);
        assert.isTrue('Ok' in res, `transfer failed: ${JSON.stringify(res)}`);
        assert.equal(await dftActor!.balanceOf(to), amountBN);
    }
});

Given(/^owner "([^"]*)" set "([^"]*)" as fee_to$/, async function (owner, feeTo) {
    logger.debug(`owner: ${owner}, feeTo: ${feeTo}`);
    const dftBasic = createDFTBasicActor(owner);
    const dftBasic2 = createDFTWithAllFeatures(owner);
    const dftBurnAble = createDFTBurnableActor(owner);
    const dftMintAble = createDFTMintableActor(owner);
    const feeToPrincipal = identity.identityFactory.getPrincipal(feeTo)!.toText();
    const feeToAccountId = identity.identityFactory.getAccountIdHex(feeTo);
    logger.debug(`feeToPrincipal: ${feeToPrincipal}`);
    const dftActors = [dftBasic, dftBasic2, dftBurnAble, dftMintAble];
    for (let i = 0; i < dftActors.length; i++) {
        const dftActor = dftActors[i];
        if (dftActor) {
            try {
                // set fee_to
                const res = await dftActor.setFeeTo(feeToPrincipal, []);
                assert.isTrue('Ok' in res, `set fee_to failed: ${JSON.stringify(res)}`);
                const result = await dftBasic.tokenInfo();
                assert.equal(result.feeTo, feeToAccountId);
            } catch {
            }
        }
    }
});

const parseToDFTInitOptions = (option: any): DFTInitOptions | undefined => {
    logger.debug(`option is ${JSON.stringify(option)}`);
    // if option is undefined, return undefined
    if (!option) return undefined;
    const decimals = parseInt(option.decimals);
    const feeDecimals = parseInt(option.rate_decimals);
    return {
        name: String(option.name),
        symbol: String(option.symbol),
        decimals: BigInt(decimals),
        totalSupply: unit.parseToOrigin(option.total_supply, decimals),
        fee: {
            minimum: Number(unit.parseToOrigin(option.fee_minimum, decimals)),
            rate: Number(option.fee_rate != 0 ? unit.parseToOrigin(option.fee_rate, feeDecimals) : 0n),
            rate_decimals: feeDecimals,
        },
        desc: [],
        owner: identity.identityFactory.getPrincipal(option.owner)!.toText(),
    };
}

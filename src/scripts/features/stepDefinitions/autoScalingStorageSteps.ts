import {When, Then} from "@cucumber/cucumber";
import {createDFTBasicActor, createStorageActor} from "~/declarations";
import {identityFactory} from "~/utils/identity";
import {parseToCommon, parseToOrigin} from "~/utils/uint";
import {assert, expect} from "chai";
import logger from "node-color-log";
import {createDFTActor} from "./utils";
import {parseRawTableToJsonArray} from "~/utils/convert";
import {OperationResult} from "~/declarations/dft_basic/dft_basic.did";

When(
    /^Check the storage canisters count is equal to "([^"]*)" ,by "([^"]*)"$/,
    async function (count, user) {
        const actor = createDFTBasicActor(user);
        const tokenInfo = await actor.tokenInfo();
        assert.equal(tokenInfo.archiveCanisters.length, parseInt(count));
    }
);

Then(/^Transfer token repeat "([^"]*)" times$/, {timeout: 10000 * 1000}, async function (repeatTimes, {rawTable}) {
    const options = parseRawTableToJsonArray(rawTable);
    for (let i = 0; i < repeatTimes; i++) {
        const transferJobs: Array<Promise<OperationResult>> = [];
        for (let j = 0; j < options.length; j++) {
            const amount = i + j;
            const option = options[j];
            const actor = createDFTBasicActor(option.from);
            const toPrincipal = identityFactory.getPrincipal(option.to)!.toText();
            const decimals = await actor.decimals();
            const amountBN = parseToOrigin(amount, decimals);
            transferJobs.push(actor.transfer([], toPrincipal, amountBN, []));
        }
        i = i + options.length - 1;
        const resArray = await Promise.all(transferJobs);
        for (const res of resArray) {
            if ("Ok" in res) {
                logger.debug(
                    `Transferring job, the block height is ${res.Ok.blockHeight}`
                );
            } else {
                expect.fail(`transfer failed, tx: ${JSON.stringify(res)}`);
            }
        }
    }
});
Then(
    /^Check the block height "([^"]*)" transfer transaction of "([^"]*)", the amount should be (\d+)$/,
    async function (blockHeight, token, amount) {
        const actor = createDFTActor(token);
        const decimals = await actor!.decimals();
        const blockHeightBN = parseToOrigin(blockHeight, 0);
        const blockRes = await actor!.blockByHeight(blockHeightBN);
        if ("Ok" in blockRes) {
            const block = blockRes.Ok;
            if ("transaction" in block) {
                const tx = block.transaction;
                if ("Transfer" in tx.operation) {
                    const transfer = tx.operation.Transfer;
                    const amountBN = parseToOrigin(amount, decimals);
                    assert.equal(amountBN, transfer.value);
                }
            }
        } else {
            expect.fail(`blockByHeight failed, tx: ${JSON.stringify(blockRes)}`);
        }
    }
);

Then(
    /^Check the block height "([^"]*)" transfer transaction of "([^"]*)", the result should be a forward result$/,
    async function (blockHeight, token) {
        const actor = createDFTActor(token);
        const decimals = await actor!.decimals();
        const blockHeightBN = parseToOrigin(blockHeight, 0);
        const blockRes = await actor!.blockByHeight(blockHeightBN);
        if ("Forward" in blockRes) {
            const canisterId = blockRes.Forward;
            const tokenInfo = await actor!.tokenInfo();
            // tokenInfo.archive_canisters should contain canisterId
            const canisterIdVal = tokenInfo.archiveCanisters.find(
                (storage) => storage === canisterId
            );
            assert.isNotNull(canisterIdVal);
        } else {
            expect.fail(`transactionByIndex failed, tx: ${JSON.stringify(blockRes)}`);
        }
    }
);

Then(
    /^Check the block height "([^"]*)" transfer transaction of "([^"]*)", the result should not be a forward result$/,
    async function (blockHeight, token) {
        const actor = createDFTActor(token);
        const blockHeightBN = parseToOrigin(blockHeight, 0);
        const tx = await actor!.blockByHeight(blockHeightBN);
        assert.isFalse("Forward" in tx);
    }
);

Then(
    /^Check the blocks query of "([^"]*)", start block height "([^"]*)",size "([^"]*)", check each transaction is correct$/,
    async function (token, startBlockHeight, size, {rawTable}) {
        const actor = createDFTActor(token);
        const decimals = await actor!.decimals();
        const optionArray = parseRawTableToJsonArray(rawTable);
        const startBlockHeightBN = parseToOrigin(startBlockHeight, 0);
        const sizeBN = parseToOrigin(size, 0);
        const queryRes = await actor!.blocksByQuery(startBlockHeightBN, sizeBN);

        optionArray.forEach((option, index) => {
            const block = queryRes.blocks[index];
            if ("transaction" in block) {
                const tx = block.transaction;
                if ("Transfer" in tx.operation) {
                    const transfer = tx.operation.Transfer;
                    const amountBN = parseToOrigin(option.amount, decimals);
                    const feeBN = parseToOrigin(option.fee, decimals);
                    assert.equal(amountBN, transfer.value);
                    assert.equal(feeBN, transfer.fee);
                }
            }
        });
    }
);

Then(/^Check token "([^"]*)"'s archives ,should be$/, async function (token, {rawTable}) {
    const actor = createDFTActor(token);
    const optionArray = parseRawTableToJsonArray(rawTable);
    const archives = await actor!.archives();

    optionArray.forEach((option, index) => {
        const archive = archives[index];
        assert.equal(archive.startBlockHeight, BigInt(option.start));
        assert.equal(archive.endBlockHeight, BigInt(option.end));
    });
});
Then(/^Get the block height "([^"]*)" transfer transaction of "([^"]*)" from archive canister, the amount should be "([^"]*)"$/, async function (blockHeight, token, amount) {
    const actor = createDFTActor(token);
    const decimals = await actor!.decimals();
    const blockHeightBN = parseToOrigin(blockHeight, 0);
    const blockRes = await actor!.blockByHeight(blockHeightBN);
    if ("Forward" in blockRes) {
        const canisterId = blockRes.Forward;
        const storageActor = createStorageActor(canisterId.toText());
        const blockInsideRes = await storageActor!.blockByHeight(blockHeightBN);
        if ("Ok" in blockInsideRes) {
            const blockInside = blockInsideRes.Ok;
            if ("transaction" in blockInside) {
                const tx = blockInside.transaction;
                if ("Transfer" in tx.operation) {
                    const transfer = tx.operation.Transfer;
                    const amountBN = parseToOrigin(amount, decimals);
                    assert.equal(amountBN, transfer.value);
                }
            }
        }

    } else {
        expect.fail(`transactionByIndex failed, tx: ${JSON.stringify(blockRes)}`);
    }
});
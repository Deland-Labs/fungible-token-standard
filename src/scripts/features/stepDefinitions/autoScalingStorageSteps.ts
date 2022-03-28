import {When, Then} from "@cucumber/cucumber";
import {createDFTBasicActor, createStorageActor} from "~/declarations";
import {identityFactory} from "~/utils/identity";
import {parseToOrigin} from "~/utils/uint";
import {assert, expect} from "chai";
import logger from "node-color-log";
import {createDFTActor} from "./utils";
import {parseRawTableToJsonArray} from "~/utils/convert";

When(/^Check the storage canisters count is equal to "([^"]*)" ,by "([^"]*)"$/, async function (count, user) {
    const actor = createDFTBasicActor(user);
    const tokenInfo = await actor.tokenInfo();
    logger.debug(`tokenInfo: ${JSON.stringify(tokenInfo)}`);
    assert.equal(tokenInfo.storages.length, parseInt(count));
});
Then(/^Transfer token "([^"]*)" from "([^"]*)" to "([^"]*)" amount of equals the times, repeat (\d+) times$/, {timeout: 6000 * 1000}, async function (token, from, to, repeatTimes) {
    const actor = createDFTBasicActor(from);
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const decimals = await actor.decimals();
    const repeat = parseInt(repeatTimes);

    for (let i = 1; i <= repeat; i++) {
        const amountBN = parseToOrigin(i, decimals);
        logger.debug(`Transferring job, the index is ${i + 1}`);
        await actor.transfer([], toPrincipal, amountBN, []);
    }
});
Then(/^Check the tx index "([^"]*)" transfer transaction of "([^"]*)", the amount should be (\d+)$/, async function (txIndex, token, amount) {
    const actor = createDFTActor(token);
    const decimals = await actor!.decimals();
    const txIndexBN = parseToOrigin(txIndex, 0);
    const tx = await actor!.transactionByIndex(txIndexBN);
    if ("Ok" in tx) {
        const txDetails = tx.Ok;
        if ("Transfer" in txDetails) {
            const txInfo = txDetails.Transfer;
            assert.equal(txInfo[0], txIndexBN);
            assert.equal(txInfo[4], parseToOrigin(amount, decimals));
        }
    } else {
        expect.fail(`transactionByIndex failed, tx: ${JSON.stringify(tx)}`);
    }
});
Then(/^Check the tx index "([^"]*)" transfer transaction of "([^"]*)", the result should be a forward result$/, async function (txIndex, token) {
    const actor = createDFTActor(token);
    const txIndexBN = parseToOrigin(txIndex, 0);
    const tx = await actor!.transactionByIndex(txIndexBN);
    if ("Forward" in tx) {
        const scalingStorageCanisterId = tx.Forward.toText();
        const storageActor = createStorageActor(scalingStorageCanisterId);
        const txInStorage = await storageActor!.transactionByIndex(txIndexBN);
        if ("Ok" in txInStorage) {
            const txDetails = txInStorage.Ok;
            if ("Transfer" in txDetails) {
                const txInfoInStorage = txDetails.Transfer;
                assert.equal(txInfoInStorage[0], txIndexBN);
            }
        } else {
            expect.fail(`transactionByIndex failed, tx: ${JSON.stringify(txInStorage)}`);
        }
    } else {
        expect.fail(`transactionByIndex in storage failed, tx: ${JSON.stringify(txInStorage)}`);
    }
});

Then(/^Check the tx index "([^"]*)" transfer transaction of "([^"]*)", the result should not be a forward result$/, async function (txIndex, token) {
    const actor = createDFTActor(token);
    const txIndexBN = parseToOrigin(txIndex, 0);
    const tx = await actor!.transactionByIndex(txIndexBN);
    assert.isFalse("Forward" in tx);
});

Then(/^Check the last (\d+) transactions of "([^"]*)", check each transaction is correct$/, async function (size, token, {rawTable}) {
    const actor = createDFTActor(token);
    const decimals = await actor!.decimals();
    const optionArray = parseRawTableToJsonArray(rawTable);
    const last10TxsRes = await actor!.lastTransactions(size);
    if ("Ok" in last10TxsRes) {
        assert.equal(last10TxsRes.Ok.length, size);
        for (let i = 0; i < size; i++) {
            const tx = last10TxsRes.Ok[i];
            const option = optionArray[i];
            if ("Transfer" in tx) {
                const transfer = tx.Transfer;
                assert.equal(transfer[0], parseToOrigin(option.index, 0), `index is not equal, expected: ${option.index}, actual: ${transfer[0]}`);
                if ("Principal" in transfer[1]) {
                    const principal = transfer[1].Principal.toText();
                    const expectedPrincipal = identityFactory.getPrincipal(option.caller)!.toText();
                    assert.equal(principal, expectedPrincipal, `caller is not equal, expected: ${expectedPrincipal}, actual: ${principal}`);
                }
                if ("Principal" in transfer[2]) {
                    const principal = transfer[2].Principal.toText();
                    const expectedPrincipal = identityFactory.getPrincipal(option.from)!.toText();
                    assert.equal(principal, expectedPrincipal, `from is not equal, expected: ${expectedPrincipal}, actual: ${principal}`);
                }
                if ("Principal" in transfer[3]) {
                    const principal = transfer[3].Principal.toText();
                    const expectedPrincipal = identityFactory.getPrincipal(option.to)!.toText();
                    assert.equal(principal, expectedPrincipal, `to is not equal, expected: ${expectedPrincipal}, actual: ${principal}`);
                }
                assert.equal(transfer[4], parseToOrigin(option.amount, decimals), `amount is not equal, expected: ${option.amount}, actual: ${transfer[4]}`);
                assert.equal(transfer[5], parseToOrigin(option.fee, decimals), `fee is not equal, expected: ${option.fee}, actual: ${transfer[5]}`);
                assert.isTrue(transfer[6] > 0, `timestamp is not greater than 0, actual: ${transfer[6]}`);
                break;
            }

        }
    } else {
        expect.fail(`lastTransactions failed, tx: ${JSON.stringify(last10TxsRes)}`);
    }
});
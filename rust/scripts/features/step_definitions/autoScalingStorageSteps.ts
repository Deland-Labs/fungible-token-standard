import {When, setDefaultTimeout, Before} from "@cucumber/cucumber";
import {createDFTBasicActor} from "~/declarations";
import {identityFactory} from "~/utils/identity";
import {parseToOrigin} from "~/utils/uint";
import {assert} from "chai";
import logger from "node-color-log";

When(/^Transfer token "([^"]*)" from "([^"]*)" to "([^"]*)" amount (\d+), repeat (\d+) times$/, {timeout: 6000 * 1000}, async function (token, from, to, amount, repeatTimes) {
    const actor = createDFTBasicActor(from);
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const decimals = await actor.decimals();
    const amountBN = parseToOrigin(amount, decimals);
    const repeat = parseInt(repeatTimes);

    for (let i = 0; i < repeat; i++) {
        logger.debug(`Transferring job, the index is ${i + 1}`);
        await actor.transfer([], toPrincipal, amountBN, []);
    }
});
When(/^Check the storage canisters count is equal to "([^"]*)" ,by "([^"]*)"$/, async function (count, user) {
    const actor = createDFTBasicActor(user);
    const tokenInfo = await actor.tokenInfo();
    logger.debug(`tokenInfo: ${JSON.stringify(tokenInfo)}`);
    assert.equal(tokenInfo.storages.length, parseInt(count));
});
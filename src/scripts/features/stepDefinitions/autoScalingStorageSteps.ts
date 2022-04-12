import { When, Then } from "@cucumber/cucumber";
import { createDFTBasicActor, createStorageActor } from "~/declarations";
import { identityFactory } from "~/utils/identity";
import { parseToOrigin } from "~/utils/uint";
import { assert, expect } from "chai";
import logger from "node-color-log";
import { createDFTActor } from "./utils";
import { parseRawTableToJsonArray } from "~/utils/convert";

When(
  /^Check the storage canisters count is equal to "([^"]*)" ,by "([^"]*)"$/,
  async function (count, user) {
    const actor = createDFTBasicActor(user);
    const tokenInfo = await actor.tokenInfo();
    logger.debug(`tokenInfo: ${JSON.stringify(tokenInfo)}`);
    assert.equal(tokenInfo.storages.length, parseInt(count));
  }
);
Then(
  /^Transfer token "([^"]*)" from "([^"]*)" to "([^"]*)" amount of equals the times, repeat (\d+) times$/,
  { timeout: 6000 * 1000 },
  async function (token, from, to, repeatTimes) {
    const actor = createDFTBasicActor(from);
    const toPrincipal = identityFactory.getPrincipal(to)!.toText();
    const decimals = await actor.decimals();
    const repeat = parseInt(repeatTimes);

    for (let i = 1; i <= repeat; i++) {
      const amountBN = parseToOrigin(i, decimals);
      const res = await actor.transfer([], toPrincipal, amountBN, []);
      if ("Ok" in res) {
        logger.debug(
          `Transferring job, the block height is ${res.Ok.blockHeight}`
        );
      }
      else{
        expect.fail(`transfer failed, tx: ${JSON.stringify(res)}`);
      }
    }
  }
);
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
      // tokenInfo.storages should contains canisterId
      const canisterIdVal = tokenInfo.storages.find(
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
  async function (token, startBlockHeight, size, { rawTable }) {
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
          const callerPrincipal = identityFactory
            .getPrincipal(option.caller)!
            .toText();
          const fromPrincipal = identityFactory
            .getPrincipal(option.from)!
            .toText();
          const toPrincipal = identityFactory.getPrincipal(option.to)!.toText();
          if ("Principal" in transfer.caller) {
            assert.equal(callerPrincipal, transfer.caller.Principal.toText());
          }
          if ("Principal" in transfer.from) {
            assert.equal(fromPrincipal, transfer.from.Principal.toText());
          }
          if ("Principal" in transfer.to) {
            assert.equal(toPrincipal, transfer.to.Principal.toText());
          }
        }
      }
    });
  }
);

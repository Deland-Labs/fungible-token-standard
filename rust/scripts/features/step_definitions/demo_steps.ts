import {Given, Then, When} from "@cucumber/cucumber";
import logger from "node-color-log";

Given(/^Step1$/,
    async function () {
        logger.info("Step1");
    });
When(/^Step2$/,
    function () {
        logger.info("Step2");
    });
Then(/^Step3$/,
    function () {
        logger.info("Step3");
    });
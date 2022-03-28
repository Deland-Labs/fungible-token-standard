import BigNumber from "bignumber.js";
const zero = new BigNumber(0);
const parseToCommon = (
    originTokenQty: bigint | string | BigNumber,
    tokenDecimals?: number,
    precision?: number
): BigNumber => {
    if (tokenDecimals === undefined) tokenDecimals = 0;
    let bn = new BigNumber(originTokenQty.toString()).div(
        new BigNumber(10).pow(tokenDecimals)
    );
    if (precision)
        bn = new BigNumber(new BigNumber(bn.toFixed(precision)).toPrecision());
    return bn;
};

const parseToOrigin = (
    commonQty: bigint | string | BigNumber | number,
    tokenDecimals: number
): bigint => {
    const bn = new BigNumber(commonQty.toString()).multipliedBy(
        new BigNumber(10).pow(tokenDecimals)
    );
    return BigInt(bn.toFormat().replaceAll(",", ""));
};

const toPrecision = (
    originTokenQty: bigint | string | BigNumber,
    precision?: number
): BigNumber => {
    let res = new BigNumber(originTokenQty.toString()).toString();
    const bn = new BigNumber(originTokenQty.toString());
    if (precision) res = new BigNumber(bn.toFixed(precision)).toPrecision();
    else res = bn.toPrecision();
    return new BigNumber(res);
};

export { zero, parseToCommon, parseToOrigin, toPrecision };

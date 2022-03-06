import {Principal} from "@dfinity/principal";
import {sha224} from "js-sha256";
import {Buffer} from "buffer";
// @ts-ignore (no type definitions for crc are available)
import crc from "crc";
import {SubAccount} from "./types";


export const principalToAccountIDInBytes = (
    principal: Principal,
    subAccount?: Uint8Array
): Uint8Array => {
    // Hash (sha224) the principal, the subAccount and some padding
    const padding = asciiStringToByteArray("\x0Aaccount-id");

    const shaObj = sha224.create();
    shaObj.update([
        ...padding,
        ...principal.toUint8Array(),
        ...(subAccount ?? Array(32).fill(0)),
    ]);
    const hash = new Uint8Array(shaObj.array());

    // Prepend the checksum of the hash and convert to a hex string
    const checksum = calculateCrc32(hash);
    return new Uint8Array([...checksum, ...hash]);
};

export const principalToAccountID = (
    principal: Principal,
    subAccount?: Uint8Array
): string => {
    const bytes = principalToAccountIDInBytes(principal, subAccount);
    return toHexString(bytes);
};


export const asciiStringToByteArray = (text: string): Array<number> => {
    return Array.from(text).map((c) => c.charCodeAt(0));
};

export const toHexString = (bytes: Uint8Array) =>
    bytes.reduce((str, byte) => str + byte.toString(16).padStart(2, "0"), "");

export const hexToBytes = (hex: string): Array<number> => {
    const bytes = new Uint8Array(hex.length / 2);
    for (let i = 0; i < hex.length; i += 2) {
        bytes[i / 2] = parseInt(hex.substr(i, 2), 16);
    }
    return Array.from(bytes);
};

export const toCandidString = (bytes: Uint8Array) => {
    // join the bytes into a string with ; as separator and the last ; removed
    const inner = bytes.reduce((str, byte) => str + byte.toString() + ";", "").slice(0, -1);
    return `vec { ${inner} }`;
}

export const toCandidString2 = (bytes: Array<number>) => {
    // join the bytes into a string with ; as separator and the last ; removed
    const inner = bytes.reduce((str, byte) => str + byte.toString() + ";", "").slice(0, -1);
    return `vec { ${inner} }`;
}

// 4 bytes
export const calculateCrc32 = (bytes: Uint8Array): Uint8Array => {
    const checksumArrayBuf = new ArrayBuffer(4);
    const view = new DataView(checksumArrayBuf);
    view.setUint32(0, crc.crc32(Buffer.from(bytes)), false);
    return Buffer.from(checksumArrayBuf);
};


export const arrayOfNumberToUint8Array = (
    numbers: Array<number>
): Uint8Array => {
    return new Uint8Array(numbers);
};

export const arrayOfNumberToArrayBuffer = (
    numbers: Array<number>
): ArrayBuffer => {
    return arrayOfNumberToUint8Array(numbers).buffer;
};

export const numberToArrayBuffer = (
    value: number,
    byteLength: number
): ArrayBuffer => {
    const buffer = new ArrayBuffer(byteLength);
    new DataView(buffer).setUint32(byteLength - 4, value);
    return buffer;
};

export const principalToSubAccount = (principal: Principal): SubAccount => {
    const bytes = principal.toUint8Array();
    const subAccount = new Uint8Array(32);
    subAccount[0] = bytes.length;
    subAccount.set(bytes, 1);
    return subAccount;
};


export const toICPe8s = (source: string): bigint => {
    if (!source) {
        return 0n;
    }
    // replace all _  to empty string
    const str = source.replace(/_/g, "").toLowerCase();
    // treat as icp if icp in string
    if (str.includes("icp")) {
        // remove icp and convert to bigint
        return BigInt(parseFloat(str.replace("icp", "")) * 100_000_000);
    } else {
        // convert to bigint
        return BigInt(str);
    }
}

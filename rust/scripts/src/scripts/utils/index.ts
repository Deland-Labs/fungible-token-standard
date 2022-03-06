export * as convert from "./convert";
export * as canister from "./canister";
export * as identity from "./identity";
export * as math from "./Result";

export const purify = (stdout: string) => {
  return stdout.replace(/(\r\n|\n|\r)/gm, "").trim();
};

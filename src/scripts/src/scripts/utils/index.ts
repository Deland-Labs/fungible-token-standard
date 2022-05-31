export * as canister from "./canister";

export const purify = (stdout: string) => {
  return stdout.replace(/(\r\n|\n|\r)/gm, "").trim();
};

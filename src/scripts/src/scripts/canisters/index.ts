import { dfxJson } from "@deland-labs/ic-dev-kit";

export const canisters = Array.from(dfxJson.get_dfx_json().canisters);
import {get_dfx_json} from "~/utils/dfxJson";

export const canisters = Array.from(get_dfx_json().canisters);
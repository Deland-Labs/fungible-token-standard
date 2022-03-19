import {get_dfx_json} from "~/utils/dfx_json";

export const canisters = Array.from(get_dfx_json().canisters);
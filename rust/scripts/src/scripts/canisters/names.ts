import {get_dfx_json} from "~/utils/dfx_json";

export const all_names = Array.from(get_dfx_json().canisters.keys());
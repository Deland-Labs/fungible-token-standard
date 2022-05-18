use candid::candid_method;
use dft_basic::service::basic_service;
use dft_types::{HttpRequest, HttpResponse};
use dft_utils::image_utils::get_image_type;
use ic_cdk_macros::query;
use json_pretty::PrettyFormatter;
use log::debug;

#[cfg_attr(coverage_nightly, no_coverage)]
#[query]
#[candid_method(query, rename = "http_request")]
fn http_request(req: HttpRequest) -> HttpResponse {
    let path = req.path().to_lowercase();
    debug!("path: {}", path);
    let cycles = ic_cdk::api::canister_balance();
    match path.as_str() {
        "/" => {
            let (token_info, metrics, total_supply) = (
                basic_service::metadata(),
                basic_service::token_metrics(),
                basic_service::total_supply(),
            );
            let fee = token_info.fee().clone();
            // convert token_info to json
            let token_info_json = format!(
                "{{name : \"{}\",symbol : \"{}\",decimals : {},totalSupply : {},fee :{{minimum: {},rate:{}%}}}}",
                token_info.name(),
                token_info.symbol(),
                token_info.decimals(),
                total_supply,
                fee.minimum,
                fee.rate as u128 * 100 / 10u128.pow(fee.rate_decimals.into())
            );

            let metrics_json = format!(
                "{{totalBlockHeight : {},localBlockCount : {},cycles : {},holders : {},allowanceSize : {}}}",
                metrics.chain_length,
                metrics.local_block_count,
                cycles,
                metrics.holders,
                metrics.allowance_size,
            );
            let json = format!("{{token:{},metrics:{}}}", token_info_json, metrics_json);
            let formatter = PrettyFormatter::from_str(json.as_str());
            let result = formatter.pretty();
            HttpResponse::ok(vec![], result.into_bytes())
        }
        "/logo" => {
            let logo = basic_service::logo().unwrap_or_default();

            // if logo is empty, return 404
            if logo.is_empty() {
                HttpResponse::not_found()
            } else {
                // if logo is not empty, mean it is a valid image
                let logo_type = get_image_type(logo.as_slice()).unwrap();

                if logo_type.is_empty() {
                    HttpResponse::not_found()
                } else {
                    HttpResponse::ok(vec![("Content-Type".into(), logo_type)], logo)
                }
            }
        }
        "/name" => {
            let name = basic_service::name();
            HttpResponse::ok(vec![], name.into_bytes())
        }
        "/symbol" => {
            let symbol = basic_service::symbol();
            HttpResponse::ok(vec![], symbol.into_bytes())
        }
        "/decimals" => {
            let decimals = basic_service::decimals();
            HttpResponse::ok(vec![], decimals.to_string().into_bytes())
        }
        "/totalsupply" => {
            let total_supply = basic_service::total_supply();
            HttpResponse::ok(vec![], total_supply.to_string().into_bytes())
        }
        _ => HttpResponse::not_found(),
    }
}

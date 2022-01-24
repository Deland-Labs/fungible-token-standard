use crate::state::TOKEN;
use crate::token::TokenStandard;
use candid::candid_method;
use dft_types::{HttpRequest, HttpResponse};
use dft_utils::get_logo_type;
use ic_cdk_macros::query;
use json_pretty::PrettyFormatter;

#[query]
#[candid_method(query, rename = "http_request")]
fn http_request(req: HttpRequest) -> HttpResponse {
    let path = req.path().to_lowercase();
    ic_cdk::api::print(format!("path: {}", path));
    let cycles = ic_cdk::api::canister_balance();
    match path.as_str() {
        "/" => {
            let (token_info, metrics) = TOKEN.with(|token| {
                let token = token.borrow();
                (token.metadata(), token.token_metrics())
            });
            // convert token_info to json
            let token_info_json = format!(
                "{{name : \"{}\",symbol : \"{}\",decimals : {},totalSupply : {},fee :{{minimum: {},rate:{}}}}}",
                token_info.name,
                token_info.symbol,
                token_info.decimals,
                token_info.total_supply,
                token_info.fee.minimum,
                format!("{} %", token_info.fee.rate * 100 / 10u64.pow(token_info.fee.rate_decimals.into()))
            );

            let metrics_json = format!(
                "{{totalTxCount : {},innerTxCount : {},cycles : {},holders : {},allowanceSize : {}}}",
                metrics.total_tx_count,
                metrics.inner_tx_count,
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
            let logo = TOKEN.with(|token| {
                let token = token.borrow();
                token.logo()
            });

            // if logo is empty, return 404
            if logo.is_empty() {
                HttpResponse::not_found()
            } else {
                // if logo is not empty, mean it is a valid image
                let logo_type = get_logo_type(&logo).unwrap();

                if logo_type.is_empty() {
                    HttpResponse::not_found()
                } else {
                    HttpResponse::ok(vec![("Content-Type".into(), logo_type.into())], logo)
                }
            }
        }
        "/name" => {
            let name = TOKEN.with(|token| {
                let token = token.borrow();
                token.name()
            });
            HttpResponse::ok(vec![], name.into_bytes())
        }
        "/symbol" => {
            let symbol = TOKEN.with(|token| {
                let token = token.borrow();
                token.symbol()
            });
            HttpResponse::ok(vec![], symbol.into_bytes())
        }
        "/decimals" => {
            let decimals = TOKEN.with(|token| {
                let token = token.borrow();
                token.decimals()
            });
            HttpResponse::ok(vec![], decimals.to_string().into_bytes())
        }
        "/totalsupply" => {
            let total_supply = TOKEN.with(|token| {
                let token = token.borrow();
                token.total_supply()
            });
            HttpResponse::ok(vec![], total_supply.to_string().into_bytes())
        }
        _ => HttpResponse::not_found(),
    }
}

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
            let (token_info, metrics, total_supply) = TOKEN.with(|token| {
                let token = token.borrow();
                (
                    token.metadata().clone(),
                    token.token_metrics().clone(),
                    token.total_supply().clone(),
                )
            });
            let fee = token_info.fee().clone();
            // convert token_info to json
            let token_info_json = format!(
                "{{name : \"{}\",symbol : \"{}\",decimals : {},totalSupply : {},fee :{{minimum: {},rate:{}}}}}",
                token_info.name(),
                token_info.symbol(),
                token_info.decimals(),
                total_supply ,
                fee.minimum,
                format!("{} %", fee.rate * 100 / 10u64.pow(fee.rate_decimals.into()))
            );

            let metrics_json = format!(
                "{{totalBlockHeight : {},localBlockCount : {},cycles : {},holders : {},allowanceSize : {}}}",
                metrics.total_block_count,
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
            let logo = TOKEN.with(|token| {
                let token = token.borrow();
                token.logo().clone().unwrap_or(vec![])
            });

            // if logo is empty, return 404
            if logo.is_empty() {
                HttpResponse::not_found()
            } else {
                // if logo is not empty, mean it is a valid image
                let logo_type = get_logo_type(logo.as_slice()).unwrap();

                if logo_type.is_empty() {
                    HttpResponse::not_found()
                } else {
                    HttpResponse::ok(
                        vec![("Content-Type".into(), logo_type.into())],
                        logo.clone(),
                    )
                }
            }
        }
        "/name" => {
            let name = TOKEN.with(|token| {
                let token = token.borrow();
                token.metadata().name().clone()
            });
            HttpResponse::ok(vec![], name.into_bytes())
        }
        "/symbol" => {
            let symbol = TOKEN.with(|token| {
                let token = token.borrow();
                token.metadata().symbol().clone()
            });
            HttpResponse::ok(vec![], symbol.into_bytes())
        }
        "/decimals" => {
            let decimals = TOKEN.with(|token| {
                let token = token.borrow();
                token.metadata().decimals().clone()
            });
            HttpResponse::ok(vec![], decimals.to_string().into_bytes())
        }
        "/totalsupply" => {
            let total_supply = TOKEN.with(|token| {
                let token = token.borrow();
                token.total_supply().clone()
            });
            HttpResponse::ok(vec![], total_supply.to_string().into_bytes())
        }
        _ => HttpResponse::not_found(),
    }
}

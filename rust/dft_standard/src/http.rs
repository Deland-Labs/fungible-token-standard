use crate::state::TOKEN;
use candid::candid_method;
use crate::token::TokenStandard;
use dft_types::{constants::FEE_RATE_DIV, HttpRequest, HttpResponse};
use ic_cdk_macros::query;
use dft_utils::get_logo_type;

#[query]
#[candid_method(query, rename = "http_request")]
fn http_request(req: HttpRequest) -> HttpResponse {
    let path = req.path().to_lowercase();
    ic_cdk::api::print(format!("path: {}", path));
    match path.as_str() {
        "/" => {
            let token_info = TOKEN.with(|token| {
                let token = token.borrow();
                token.metadata()
            });
            // convert token_info to json
            let token_info_json = format!("{{\n  name : \"{}\",\n  symbol : \"{}\",\n  decimals : {},\n  totalSupply : {},\n  fee :\n  {{\n    minimum: {},\n    rate:{}\n  }}\n}}",
                                          token_info.name,
                                          token_info.symbol,
                                          token_info.decimals,
                                          token_info.total_supply,
                                          token_info.fee.minimum,
                                          token_info.fee.rate * 100 / FEE_RATE_DIV
            );
            HttpResponse::ok(vec![], token_info_json.into_bytes())
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
use crate::state::TOKEN;
use candid::candid_method;
use dft_standard::token::TokenStandard;
use dft_types::{constants::FEE_RATE_DIV, HttpRequest, HttpResponse};
use ic_cdk_macros::query;

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
            let token_info_json = format!("{{\n  name : {},\n  symbol : {},\n  decimals : {},\n  totalSupply : {},\n  fee :\n  {{\n    minimum: {},\n    rate:{}\n  }}\n}}",
            token_info.name,
            token_info.symbol,
            token_info.decimals,
            token_info.total_supply,
            token_info.fee.minimum,
            token_info.fee.rate*100/FEE_RATE_DIV
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
                // get the first byte of logo , if it is 0xFF, it means the logo is png, otherwise it is jpg

                let mut logo_type = "";
                for &(k, v) in MAGIC_BYTES.iter() {
                    if logo.len() > k.len() {
                        if &logo[0..k.len()] == k {
                            logo_type = v;
                            break;
                        }
                    }
                }

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

static MAGIC_BYTES: [(&[u8], &str); 21] = [
    (b"\x89PNG\r\n\x1a\n", "image/png"),
    (&[0xff, 0xd8, 0xff], "image/jpg"),
    (b"GIF89a", "image/gif"),
    (b"GIF87a", "image/gif"),
    (b"RIFF", "image/webp"), // TODO: better magic byte detection, see https://github.com/image-rs/image/issues/660
    (b"MM\x00*", "image/tiff"),
    (b"II*\x00", "image/tiff"),
    (b"DDS ", "image/vnd.ms-dds"),
    (b"BM", "image/bmp"),
    (&[0, 0, 1, 0], "image/ico"),
    (b"#?RADIANCE", "image/x-hdr"),
    (b"P1", "image/x-portable-bitmap"),
    (b"P2", "image/x-portable-bitmap"),
    (b"P3", "image/x-portable-bitmap"),
    (b"P4", "image/x-portable-bitmap"),
    (b"P5", "image/x-portable-bitmap"),
    (b"P6", "image/x-portable-bitmap"),
    (b"P7", "image/x-portable-bitmap"),
    (b"farbfeld", "image/farbfeld"),
    (b"\0\0\0 ftypavif", "image/avif"),
    (b"\0\0\0\x1cftypavif", "image/avif"),
];

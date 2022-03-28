use std::collections::HashMap;

#[allow(dead_code)]
use candid::{CandidType, Deserialize, Func};
use serde_bytes::ByteBuf;
type HeaderField = (String, String);
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Token {}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: Vec<(String, String)>,
    pub body: ByteBuf,
}

impl HttpRequest {
    //   path
    pub fn path(&self) -> String {
        let url = &self.url;
        let parts: Vec<&str> = url.split('?').collect();
        parts[0].to_string()
    }
    // get params
    pub fn params(&self) -> HashMap<String, String> {
        let url = &self.url;
        let parts: Vec<&str> = url.split('?').collect();
        let params: Vec<&str> = parts[1].split('&').collect();
        let mut result = HashMap::new();
        for param in params {
            let parts: Vec<&str> = param.split('=').collect();
            let key = parts[0].to_string();
            let value = parts[1].to_string();
            result.insert(key, value);
        }
        result
    }
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub enum StreamingStrategy {
    Callback { callback: Func, token: Token },
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct HttpResponse {
    pub status_code: u16,
    pub headers: Vec<HeaderField>,
    #[serde(with = "serde_bytes")]
    pub body: Vec<u8>,
    pub streaming_strategy: Option<StreamingStrategy>,
}

impl HttpResponse {
    fn new(status_code: u16, headers: Vec<(String, String)>, body: Vec<u8>) -> HttpResponse {
        HttpResponse {
            status_code: status_code,
            headers: headers,
            body: body,
            streaming_strategy: None,
        }
    }
    pub fn ok(headers: Vec<(String, String)>, body: Vec<u8>) -> HttpResponse {
        HttpResponse::new(200, HttpResponse::merge_default_headers(headers), body)
    }    
    pub fn bad_request() -> HttpResponse {
        HttpResponse::new(400, HttpResponse::default_headers(), vec![])
    }
    pub fn unauthorized() -> HttpResponse {
        HttpResponse::new(401, HttpResponse::default_headers(), vec![])
    }
    pub fn forbidden() -> HttpResponse {
        HttpResponse::new(403, HttpResponse::default_headers(), vec![])
    }
    pub fn not_found() -> HttpResponse {
        HttpResponse::new(404, HttpResponse::default_headers(), vec![])
    }
    pub fn internal_server_error() -> HttpResponse {
        HttpResponse::new(500, HttpResponse::default_headers(), vec![])
    }

    pub fn default_headers() -> Vec<(String, String)> {
        vec![
            ("Access-Control-Allow-Origin".into(), "*".into()),
            ("Content-Type".into(), "application/json".into()),
            ("Power-By".into(), "Deland Labs".into()),
        ]
    }

    fn merge_default_headers(headers: Vec<(String, String)>) -> Vec<(String, String)> {
        let mut result = HttpResponse::default_headers();
        for header in headers {
            result.push(header);
        }
        result
    }
}

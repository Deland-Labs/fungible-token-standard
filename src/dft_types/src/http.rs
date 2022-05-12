use candid::{CandidType, Deserialize, Func};
use serde_bytes::ByteBuf;
use std::collections::HashMap;

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
            status_code,
            headers,
            body,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_default_headers() {
        let headers = vec![("test_header".into(), "123".into())];
        let result = HttpResponse::merge_default_headers(headers);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].0, "Access-Control-Allow-Origin");
        assert_eq!(result[0].1, "*");
        assert_eq!(result[1].0, "Content-Type");
        assert_eq!(result[1].1, "application/json");
        assert_eq!(result[2].0, "Power-By");
        assert_eq!(result[2].1, "Deland Labs");
        assert_eq!(result[3].0, "test_header");
        assert_eq!(result[3].1, "123");
    }

    #[test]
    fn test_default_headers() {
        let result = HttpResponse::default_headers();
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].0, "Access-Control-Allow-Origin");
        assert_eq!(result[0].1, "*");
        assert_eq!(result[1].0, "Content-Type");
        assert_eq!(result[1].1, "application/json");
        assert_eq!(result[2].0, "Power-By");
        assert_eq!(result[2].1, "Deland Labs");
    }

    #[test]
    fn test_ok() {
        let result = HttpResponse::ok(vec![], vec![]);
        assert_eq!(result.status_code, 200);
        assert_eq!(result.headers.len(), 3);
    }

    #[test]
    fn test_bad_request() {
        let result = HttpResponse::bad_request();
        assert_eq!(result.status_code, 400);
        assert_eq!(result.headers.len(), 3);
    }

    #[test]
    fn test_unauthorized() {
        let result = HttpResponse::unauthorized();
        assert_eq!(result.status_code, 401);
        assert_eq!(result.headers.len(), 3);
    }

    #[test]
    fn test_forbidden() {
        let result = HttpResponse::forbidden();
        assert_eq!(result.status_code, 403);
        assert_eq!(result.headers.len(), 3);
    }

    #[test]
    fn test_not_found() {
        let result = HttpResponse::not_found();
        assert_eq!(result.status_code, 404);
        assert_eq!(result.headers.len(), 3);
    }

    #[test]
    fn test_internal_server_error() {
        let result = HttpResponse::internal_server_error();
        assert_eq!(result.status_code, 500);
        assert_eq!(result.headers.len(), 3);
    }

    #[test]
    fn test_default_headers_merge() {
        let headers = vec![("test_header".into(), "123".into())];
        let result = HttpResponse::merge_default_headers(headers);
        assert_eq!(result.len(), 4);
        assert_eq!(result[0].0, "Access-Control-Allow-Origin");
        assert_eq!(result[0].1, "*");
        assert_eq!(result[1].0, "Content-Type");
        assert_eq!(result[1].1, "application/json");
        assert_eq!(result[2].0, "Power-By");
        assert_eq!(result[2].1, "Deland Labs");
        assert_eq!(result[3].0, "test_header");
        assert_eq!(result[3].1, "123");
    }

    #[test]
    fn test_req_path() {
        let req = HttpRequest {
            method: "".to_string(),
            url: "/test/path?test=123".to_string(),
            headers: vec![],
            body: Default::default(),
        };
        assert_eq!(req.path(), "/test/path");
    }

    #[test]
    fn test_req_params() {
        let req = HttpRequest {
            method: "".to_string(),
            url: "123.com/test/path?test=123".to_string(),
            headers: vec![],
            body: Default::default(),
        };

        let result = req.params();
        assert_eq!(result.len(), 1);
        assert_eq!(result["test"], "123");
    }
}

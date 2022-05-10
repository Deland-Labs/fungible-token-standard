use candid::{CandidType, Deserialize};
use serde::Serialize;
use std::collections::HashMap;

use crate::StableState;

const OFFICIAL_SITE: &str = "OFFICIAL_SITE";
const MEDIUM: &str = "MEDIUM";
const OFFICIAL_EMAIL: &str = "OFFICIAL_EMAIL";
const DESCRIPTION: &str = "DESCRIPTION";
const BLOG: &str = "BLOG";
const REDDIT: &str = "REDDIT";
const SLACK: &str = "SLACK";
const FACEBOOK: &str = "FACEBOOK";
const TWITTER: &str = "TWITTER";
const GITHUB: &str = "GITHUB";
const TELEGRAM: &str = "TELEGRAM";
const WECHAT: &str = "WECHAT";
const LINKEDIN: &str = "LINKEDIN";
const DISCORD: &str = "DISCORD";
const WHITE_PAPER: &str = "WHITE_PAPER";

const DSCVR: &str = "DSCVR";
const OPENCHAT: &str = "OPENCHAT";
const DISTRIKT: &str = "DISTRIKT";

const DESC_KEYS: [&str; 18] = [
    DSCVR,
    OPENCHAT,
    DISTRIKT,
    OFFICIAL_SITE,
    MEDIUM,
    OFFICIAL_EMAIL,
    DESCRIPTION,
    BLOG,
    REDDIT,
    SLACK,
    FACEBOOK,
    TWITTER,
    GITHUB,
    TELEGRAM,
    WECHAT,
    LINKEDIN,
    DISCORD,
    WHITE_PAPER,
];

#[derive(CandidType, Clone, Default, Debug, Deserialize, Serialize)]
pub struct TokenDescription {
    desc: HashMap<String, String>,
}

impl TokenDescription {
    pub fn new() -> Self {
        TokenDescription {
            desc: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<String> {
        self.desc.get(key).cloned()
    }

    pub fn set(&mut self, key: String, value: String) {
        if DESC_KEYS.contains(&key.as_str()) {
            self.desc.insert(key, value);
        }
    }

    pub fn get_all(&self) -> HashMap<String, String> {
        self.desc.clone()
    }

    pub fn set_all(&mut self, desc: HashMap<String, String>) {
        for (key, value) in desc {
            self.set(key, value)
        }
    }
    pub fn to_vec(&self) -> Vec<(String, String)> {
        self.desc
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
    pub fn restore_from(&mut self, vec: Vec<(String, String)>) {
        self.desc = HashMap::new();
        for (k, v) in vec {
            self.desc.insert(k, v);
        }
    }
}

impl StableState for TokenDescription {
    fn encode(&self) -> Vec<u8> {
        bincode::serialize(&self.desc).unwrap()
    }

    fn decode(bytes: Vec<u8>) -> Result<Self, String> {
        let desc: HashMap<String, String> = bincode::deserialize(&bytes).unwrap();

        Ok(TokenDescription { desc })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_description() {
        let mut desc = TokenDescription::new();
        desc.set("key".to_string(), "value".to_string());
        assert_eq!(desc.get("key"), None);
        desc.set(TWITTER.to_string(), "value".to_string());
        assert_eq!(desc.get(TWITTER), Some("value".to_string()));
        desc.set(FACEBOOK.to_string(), "value2".to_string());
        assert_eq!(desc.get(FACEBOOK), Some("value2".to_string()));

        let map = desc.get_all();
        assert_eq!(map.len(), 2);
        assert!(map.contains_key(TWITTER));
        assert!(map.contains_key(FACEBOOK));
    }

    #[test]
    fn test_token_description_restore() {
        let mut desc = TokenDescription::new();

        let mut descs: Vec<(String, String)> = Vec::new();
        descs.push((TWITTER.to_string(), "value".to_string()));
        descs.push((FACEBOOK.to_string(), "value2".to_string()));
        desc.restore_from(descs);
        let map = desc.get_all();
        assert_eq!(map.len(), 2);
        assert!(map.contains_key(TWITTER));
        assert!(map.contains_key(FACEBOOK));
    }

    #[test]
    fn test_token_description_encode_decode() {
        let mut desc = TokenDescription::new();
        desc.set(TWITTER.to_string(), "value".to_string());
        desc.set(FACEBOOK.to_string(), "value2".to_string());
        let vec = desc.get_all();
        let desc2 = TokenDescription::decode(desc.encode()).unwrap();
        let vec2 = desc2.get_all();
        assert_eq!(vec, vec2);
    }

    #[test]
    fn test_token_description_to_vec() {
        let mut desc = TokenDescription::new();
        desc.set(TWITTER.to_string(), "value".to_string());
        desc.set(FACEBOOK.to_string(), "value2".to_string());
        let vec = desc.to_vec();
        let mut desc2 = TokenDescription::new();
        desc2.restore_from(vec.clone());
        let vec2 = desc2.to_vec();
        for (k, v) in vec {
            assert!(vec2.contains(&(k, v)));
        }
    }

    #[test]
    fn test_token_description_set_all() {
        let mut desc = TokenDescription::new();
        let mut values: HashMap<String, String> = HashMap::new();
        values.insert(TWITTER.to_string(), "value".to_string());
        values.insert(FACEBOOK.to_string(), "value2".to_string());
        desc.set_all(values);
        let map = desc.get_all();
        assert_eq!(map.len(), 2);
        assert!(map.contains_key(TWITTER));
        assert!(map.contains_key(FACEBOOK));
    }
}

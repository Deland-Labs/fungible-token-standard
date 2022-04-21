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
const WEACT: &str = "WEACT";

const DESC_KEYS: [&str; 19] = [
    DSCVR,
    OPENCHAT,
    DISTRIKT,
    WEACT,
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

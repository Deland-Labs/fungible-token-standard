use std::collections::HashMap;

use candid::{CandidType, Deserialize};

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

#[derive(CandidType, Clone, Default, Debug, Deserialize)]
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
        self.desc = desc;
    }
    pub fn to_vec(&self)-> Vec<(String, String)> {
        self.desc.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}

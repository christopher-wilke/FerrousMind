use std::collections::{HashSet, HashMap};
use crate::constants::REGX;

pub fn split_words_from_raw(raw_text: &String) -> Vec<&str> {
    REGX.split(&raw_text)
        .map(|s| s.unwrap())
        .collect()
}

pub fn create_hs_from_raw(raw_text: String) -> HashSet<String> {
    REGX.split(&raw_text)
        .filter_map(|res| res.ok())
        .filter(|&r| !r.is_empty())
        .map(|v| v.to_string())
        .collect::<HashSet<String>>()
}

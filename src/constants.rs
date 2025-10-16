use lazy_static::lazy_static;
use fancy_regex::Regex;

lazy_static!(
    pub static ref REGX: Regex = Regex::new(r#"([,.?_!"()']|--|\s)"#).unwrap();
);

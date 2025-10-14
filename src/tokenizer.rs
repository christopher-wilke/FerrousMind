use std::collections::{HashSet, HashMap};
use fancy_regex::Regex;

pub struct Tokenzier {
    id_hm: HashMap<String, i32>
}

impl Tokenzier {

    pub fn new() -> Self {
        Tokenzier { id_hm: HashMap::new() }
    }

    pub fn encode(&mut self, set: HashSet<String>) -> &HashMap<String, i32> {
        self.id_hm = set.iter()
            .enumerate()
            .map(|(i, token)| (token.to_string(), i as i32))
            .collect();

        &self.id_hm
    }   


    // We should move this to the File struct
    pub fn extract_words(raw_text: String) -> HashSet<String> {
        let expr = r#"([,.?_!"()']|--|\s)"#;

        let re = Regex::new(expr).unwrap();

        re.split(&raw_text)
            .filter_map(|res| res.ok())
            .filter(|&r| !r.is_empty())
            .map(|v| v.to_string())
            .collect::<HashSet<String>>()
    }
}

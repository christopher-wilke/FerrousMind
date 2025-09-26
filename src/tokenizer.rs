use fancy_regex::Regex;

pub struct Tokenzier {
    content: String
}

impl Tokenzier {

    pub fn new(content: String) -> Self {
        Self {
            content
        }
    }
    
    pub fn extract_sorted_words(&self) -> Vec<&str> {
        let re = Regex::new(r#"([,.?_!"()']|--|\s)"#)
            .unwrap();

        let mut preprocessed: Vec<&str> = re
            .split(&self.content[..])
            .filter_map(|x|
                x
                    .ok()
                    .filter(|s| !s.is_empty())
            )
            .collect();

        preprocessed.sort();
        preprocessed
    }
}

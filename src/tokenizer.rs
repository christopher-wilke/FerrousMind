use std::collections::{HashSet, HashMap};
use crate::helpers::create_hs_from_raw;

pub struct Tokenzier {
    id_hm: HashMap<String, i32>
}

impl Tokenzier {

    pub fn new(set: HashSet<String>) -> Self {
        let id_hm = set.iter()
            .enumerate()
            .map(|(i, token)| (token.to_string(), i as i32))
            .collect();

        Tokenzier { id_hm }
    }

    pub fn encode(&self, words: Vec<&str>) {
        // let mut res  = vec![];

        

        // for w in words {

        //     println!("{w}");
        // }
        
        // self.id_hm = set.iter()
        //     .enumerate()
        //     .map(|(i, token)| (token.to_string(), i as i32))
        //     .collect();

        // &self.id_hm
    }   
}

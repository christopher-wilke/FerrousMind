use std::{any::Any, collections::{HashMap, HashSet}};

pub struct Tokenzier {
    str_to_id: HashMap<String, u32>,
    id_to_str: HashMap<u32, String>
}

impl Tokenzier {

    pub fn new(set: HashSet<String>) -> Self {

        let mut str_to_id = HashMap::new();
        let mut id_to_str = HashMap::new();

        for (i, v) in set.into_iter().enumerate() {
            str_to_id.insert(v.clone(), i as u32);
            id_to_str.insert(i as u32, v);
        }

        Tokenzier { str_to_id, id_to_str }
    }

    pub fn encode<'a, I>(&self, words: I) -> Vec<u32>
    where I: IntoIterator<Item=&'a str>
    {
        words.into_iter()
            .map(|word| *self.str_to_id.get(word)
            .expect("Word not found in hashmap"))
            .collect()
    }

    pub fn decode<I>(&self, ids: I) -> String
    where I: IntoIterator<Item = u32>
    {
        ids.into_iter()
            .filter_map(|id| {
                self.id_to_str
                    .get(&id)
                    .map(|x| x.as_str())
            })
            .collect::<Vec<&str>>()
            .join(" ")
    }
}

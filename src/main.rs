mod constants;
mod file;
mod helpers;
mod tokenizer;

use tokenizer::Tokenzier;
use file::File;
use helpers::create_hs_from_raw;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Converts the content of a file to a HashSet
    // let words = Tokenzier::extract_words(File::read_content()?);

    let simple_test_string = "The brown dog playfully chased the swift fox".to_string();
    let words = crate::helpers::split_words_from_raw(&simple_test_string);
    let hs_words = create_hs_from_raw(simple_test_string.clone());

    let tokenizer = Tokenzier::new(hs_words);
    
    tokenizer.encode(words);
    // let id_hm = tokenizer.encode(words);

    // for (idx, val) in id_hm {
    //     println!("{}: {}", idx, val);
    // }
    
    Ok(())
}

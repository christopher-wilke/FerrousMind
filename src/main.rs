mod file;
mod tokenizer;

use tokenizer::Tokenzier;
use file::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // converts the content of a file to a HashSet
    // let words = Tokenzier::extract_words(File::read_content()?);
    //
    let mut tokenizer = Tokenzier::new(); 

    let simple_test_string = "The brown dog playfully chased the swift fox".to_string();
    let words = Tokenzier::extract_words(simple_test_string);
    let id_hm = tokenizer.encode(words);

    for (idx, val) in id_hm {
        println!("{}: {}", idx, val);
    }
    
    Ok(())
}

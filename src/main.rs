mod file;

use fancy_regex::Regex;
use file::File;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let raw_text = File::read_content()?;

    let re = Regex::new(r#"([,.?_!"()']|--|\s)"#)
        .unwrap();

    let mut preprocessed: Vec<&str> = re
        .split(&raw_text[..])
        .filter_map(|x|
            x
                .ok()
                .filter(|s| !s.is_empty())
        )
        .collect();

    preprocessed.sort();

    println!("{:?}", preprocessed);
    
    Ok(())
}

use fancy_regex::Regex;

fn main() {
    let raw_text = "Hello, this is a test!".to_string();

    let re = Regex::new(r#""#)
        .unwrap();

    let preprocessed1: Vec<&str> = re.split(&raw_text[..]).map(|x| x.unwrap())
        .collect();

    println!("{}", raw_text);
}

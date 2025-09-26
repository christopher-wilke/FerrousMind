use std::fs;

const FILE_PATH: &'static str = "data/the-verdict.txt";

#[derive(Debug)]
pub enum FileError {
    CouldNotReadError(String)
}

impl std::error::Error for FileError {}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::CouldNotReadError(msg) => write!(f, "FileError: {}", msg)
        }
    }
}

pub struct File; 
    
impl File {
    pub fn read_content() -> Result<String, FileError> {
        fs::read_to_string(FILE_PATH)
            .map_err(|_| FileError::CouldNotReadError(format!("could not read '{FILE_PATH}'")))
    }   
}

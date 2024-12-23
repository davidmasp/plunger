
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PlungerError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Reached line limit without a match")]
    LineLimitReached,
}

// get name line from file
pub fn get_name_line(file_path: &std::path::Path, limit_lines: &usize, regex: &Regex) -> Result<String, PlungerError> {
    let f: File = File::open(file_path)?; // this is the std::io::Error error
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    let mut count = 0;

    while &count < limit_lines {
        line.clear();
        let _len = reader.read_line(&mut line)?; // also std::io::Error
        
        let regex_captures = regex.captures(&line);
        match regex_captures {
            Some(captures) => {
                if captures.len() == 2 {
                    // 0 is the full match, 1 is the first capture group
                    return Ok(captures[1].to_string());
                } else {
                    // I think this should never happen
                    println!("{:?}", captures);
                    panic!("Regex capture group count is not 1");
                }
            },
            None => {
                count += 1;
                continue;
            },
        }
    }
    return Err(PlungerError::LineLimitReached);
}


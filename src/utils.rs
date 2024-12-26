
use regex::Regex;

pub fn extract_regex_simple(regex: &Regex, line: &String) -> Option<String> {
    // currently, this needs to be 1
    let param_n_captures = 1;
    let total_captures_len = param_n_captures + 1;
    let regex_captures = regex.captures(&line);
    match regex_captures {
        Some(captures) => {
            if captures.len() == total_captures_len {
                let out = captures[1].to_string();
                return Some(out);
            } else {
                // I think this should never happen
                println!("{:?}", captures);
                panic!("Regex capture group count is not 1");
            }
        },
        None => {
            return None
        },
    }
}

pub fn extract_regex_two(regex: &Regex, line: &String) -> Option<(String, String)> {
    // currently, this needs to be 1
    let param_n_captures = 2;
    let total_captures_len = param_n_captures + 1;
    let regex_captures = regex.captures(&line);
    match regex_captures {
        Some(captures) => {
            if captures.len() == total_captures_len {
                let out1 = captures[1].to_string();
                let out2 = captures[2].to_string();
                return Some((out1, out2));
            } else {
                // I think this should never happen
                println!("{:?}", captures);
                panic!("Regex capture group count is not 2");
            }
        },
        None => {
            return None
        },
    }
}


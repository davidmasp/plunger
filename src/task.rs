

use fs_extra::dir::get_size;
use ignore::WalkBuilder;
use std::collections::HashMap;
use regex::Regex;
use human_repr::HumanCount;

use log::info;
use log::debug;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use thiserror::Error;

use crate::utils::extract_regex_simple;

#[derive(Error, Debug)]
pub enum PlungerTaskError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Reached line limit without a match")]
    LineLimitReached,
}

// get name line from file
pub fn get_name_line(file_path: &std::path::Path, limit_lines: &usize, regex: &Regex) -> Result<String, PlungerTaskError> {
    let f: File = File::open(file_path)?; // this is the std::io::Error error
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    let mut count = 0;

    while &count < limit_lines {
        line.clear();
        let _len = reader.read_line(&mut line)?; // also std::io::Error

        match extract_regex_simple(regex, &line){
            Some(capt) => {
                return Ok(capt);
            },
            None => {
                count += 1;
                continue;
            },
        }
    }
    Err(PlungerTaskError::LineLimitReached)
}

pub fn workflow_task(params_restring: &str, params_target_workdir: &str, params_target_filename:&str, params_limit_lines: usize, params_tasks_to_delete: Vec<String>) -> Result<(), PlungerTaskError> {

    let re_result = Regex::new(params_restring);
    let re = match re_result {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Error creating regex: {:?}", e);
            std::process::exit(1);
        }
    };

    let mut task_map: HashMap<String, u64> = HashMap::new();
    let mut total_liberated_space: u64 = 0;

    for result in WalkBuilder::new(params_target_workdir).hidden(false).build() {
        let dir_element = match result{
            Ok(test) => test,
            Err(_) => continue,
        };

        let fname_option = dir_element.path().file_name();
        let dirname_option = dir_element.path().parent();

        match (fname_option, dirname_option){
            (Some(name), Some(dirname)) => {
                if name == params_target_filename {

                    let folder_size_res = get_size(dirname);
                    let folder_size = match folder_size_res {
                        Ok(size) => size,
                        Err(e) => {
                            eprintln!("Error getting folder size: {:?}", e);
                            std::process::exit(1);
                        }
                    };

                    let task_name_result = get_name_line(dir_element.path(), &params_limit_lines, &re);
                    let task_name = match task_name_result {
                        Ok(name) => name,
                        Err(e) => {
                            eprintln!("Error getting task name: {:?}", e);
                            std::process::exit(1);
                        }
                    };
                    
                    let count = task_map.entry(task_name.to_string()).or_insert(0);
                    *count += folder_size;

                    debug!("{:?} {} // {}\n {}", name, folder_size, dirname.display(), task_name);
                    
                    if params_tasks_to_delete.contains(&task_name) {
                        debug!("Deleting {:?}", dirname);
                        let rmres = std::fs::remove_dir_all(dirname);
                        match rmres {
                            Ok(_) => {
                                total_liberated_space += folder_size;
                                debug!("Deleted {:?}", dirname);
                            },
                            Err(e) => {
                                eprintln!("Error deleting {:?}: {:?}", dirname, e);
                                std::process::exit(1);
                            } 
                        }
                    }
                } else {
                    continue;
                }
            },
            (None, None) => continue,
            (Some(_), None) => continue,
            (None, Some(_)) => continue, // all cases will need a path and a file! 
        }
    }

    for (k,v) in task_map.iter() {
        // leaving this like this now, later wanna do a 
        // a json or HTML output from this!
        println!("{} ➡️ {}", k, v.human_count_bytes());
    }

    info!("Total space liberated: {}", total_liberated_space.human_count_bytes());
    Ok(())
}

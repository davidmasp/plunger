
use crate::utils::{extract_regex_simple, extract_regex_two};
use std::path::Path;
use regex::Regex;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use thiserror::Error;
use ignore::WalkBuilder;

use fs_extra::dir::get_size;
use human_repr::HumanCount;
/* 
use std::collections::HashMap;
use human_repr::HumanCount;
*/

use log::info;
use log::debug;

#[derive(Error, Debug)]
pub enum PlungerRunError {
    #[error("IO error")]
    Io(#[from] std::io::Error),
    #[error("Missing info in log file")]
    MissingInfoInLog,
    #[error("Error extracting strings from directory")]
    ErrorDirString,
}

#[derive(Debug)]
pub enum TaskType {
    Cached,
    Submitted,
}

#[derive(Debug)]
pub struct Task {
    pub _name: String,
    pub _task_type: TaskType,
    pub _mini_path: String,
}

#[derive(Debug)]
pub struct Run {
    pub code: String,
    pub work_dir: std::path::PathBuf,
    pub log_path: std::path::PathBuf,
    pub tasks: Vec<Task>,
    pub minipaths: Vec<String>, // use Set because it's indexed
}

impl Run {
    pub fn new_frompath(path: &str, regex_code: &Regex, regex_wdir: &Regex, regex_task_cached: &Regex, regex_task_submitted: &Regex) -> Result<Run, PlungerRunError> {
        let file_path = std::path::PathBuf::from(path);
        let log_path = file_path.clone();
        let f: File = File::open(file_path)?; // this is the std::io::Error error
        let mut reader = BufReader::new(f);
        let mut line = String::new();

        let mut wkdir = None;
        let mut code = None;
        let mut tasks: Vec<Task> = vec![];
        let mut minipaths: Vec<String> = vec![];

        loop {
            let len = reader.read_line(&mut line)?;
            if len == 0 {  // Stream is empty
                break;
            }

            if code.is_none() {
                code = extract_regex_simple(regex_code, &line);
            }

            if wkdir.is_none() {
                wkdir = extract_regex_simple(regex_wdir, &line);
            }

            if let Some((cached_dir, cached_task_name)) = extract_regex_two(regex_task_cached, &line) {
                tasks.push(Task {
                    _name: cached_task_name,
                    _task_type: TaskType::Cached,
                    _mini_path: cached_dir.clone(),
                });
                minipaths.push(cached_dir);
            } else if let Some((submitted_dir, submitted_task_name)) = extract_regex_two(regex_task_submitted, &line) {
                tasks.push(Task {
                    _name: submitted_task_name,
                    _task_type: TaskType::Submitted,
                    _mini_path: submitted_dir.clone(),
                });
                minipaths.push(submitted_dir);
            } 
            
            line.clear();
        }

        match (code, wkdir) {
            (Some(cd), Some(wd)) => {
                Ok(Run {
                    code: cd,
                    work_dir: std::path::PathBuf::from(wd),
                    log_path,
                    tasks,
                    minipaths,
                })
            },
            _ => Err(PlungerRunError::MissingInfoInLog),
        }
    }
}

pub fn workflow_run(params_path_log: &str, params_cached: &str, params_submitted: &str, params_runname: &str, params_wkdirre: &str, force: &bool) -> Result<(), PlungerRunError> {

    let re_cached = Regex::new(params_cached).unwrap();
    let re_submitted = Regex::new(params_submitted).unwrap();
    let re_runname = Regex::new(params_runname).unwrap();
    let re_wkdirre = Regex::new(params_wkdirre).unwrap();

    let run_obj = Run::new_frompath(
        params_path_log,
        &re_runname,
        &re_wkdirre,
        &re_cached,
        &re_submitted,
    )?;

    debug!("Code: {}", run_obj.code);
    debug!("Workdir: {}", run_obj.work_dir.to_str().expect("weird error, workdir"));
    debug!("Logpath: {}", run_obj.log_path.to_str().expect("weird error, logpath"));
    debug!("Tasks: {}", run_obj.tasks.len());

    let mut purged_size = 0;

    let walker = WalkBuilder::new(&run_obj.work_dir)
        .max_depth(Some(2))
        .parents(false)
        .follow_links(false)
        .build();
        // for later .exclude(["singularity"]) conda etc.
        // types-

    for result in walker {
        // debug!("Result: {:?}", result);
        let dir_element = match result{
            Ok(test) => test,
            Err(_) => continue,
        };
        if run_obj.work_dir  == dir_element.path() {
            debug!("Skipping workdir");
            continue;
        }
        let fname_option = dir_element.path().file_name();
        let dirname_option = dir_element.path().parent();
        match (fname_option, dirname_option){
            (Some(fpath), Some(dir)) => {

                if run_obj.work_dir == dir {
                    debug!("Skipping first level directories");
                    continue;
                }

                let fpath_str = match fpath.to_str() {
                    Some(s) => s,
                    None => {
                        return Err(PlungerRunError::ErrorDirString);
                    },
                };

                debug!("folder: {:?}", fpath);
                let main_dir = dir.file_name()
                    .and_then(|os_str| os_str.to_str())
                    .unwrap_or("");
                // probably put more protection here?

                if main_dir.len() != 2 {
                    debug!("Skipping non-2 level directories");
                    continue;
                }

                // this might FAIL?
                debug!("main folder: {}", main_dir);
                let last_dir_6char = &fpath_str[0..6];
                let mini_path = format!("{}/{}", main_dir, last_dir_6char);

                let file_path = Path::new(fpath);
                let full_path =  dir.join(file_path);
                let full_path_str = match full_path.to_str() {
                    Some(s) => s,
                    None => {
                        return Err(PlungerRunError::ErrorDirString);
                    },
                };

                if run_obj.minipaths.contains(&mini_path) {
                    // do not delete
                    debug!("DO NOT DELETE: {}", full_path_str);
                    
                } else {
                    debug!("DELETE: {}", full_path_str);

                    let folder_size = match get_size(full_path_str) {
                        Ok(size) => size,
                        Err(e) => {
                            eprintln!("Error getting folder size: {:?}", e);
                            std::process::exit(1);
                        }
                    };
                    purged_size += folder_size;

                    if *force {
                        // delete
                        let rmres = std::fs::remove_dir_all(full_path_str);
                        match rmres {
                            Ok(_) => {
                                // total_liberated_space += folder_size;
                                debug!("Deleted {:?}", full_path_str);
                            },
                            Err(e) => {
                                eprintln!("Error deleting {:?}: {:?}", full_path_str, e);
                                std::process::exit(1);
                            }
                        }
                    }
                }
            },
            _ => {
                continue;
            }
        }
    }
    
    if *force {
        info!("Purged size: {}", purged_size.human_count_bytes());
    } else {
        info!("Would purge size: {}", purged_size.human_count_bytes());
    }
    Ok(())
}

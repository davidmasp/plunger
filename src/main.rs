
mod plunger;
use plunger::get_name_line;

use fs_extra::dir::get_size;
use ignore::WalkBuilder;
use std::collections::HashMap;
use regex::Regex;
use human_repr::HumanCount;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Workdir to parse
    #[arg(short = 'w', long, default_value = "./work")]
    workdir: String,

    /// Tasks that will be deleted
    #[arg(short = 't', long, value_delimiter = ' ')]
    tasks: Option<Vec<String>>,

    /// Maximum lines to check in each .command.run file
    #[arg(short = 'l', long, default_value_t = 30)]
    limit_lines: usize,

    /// Debug output
    #[arg(short, long)]
    debug: bool,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

use log::info;
use log::debug;
use simplelog;
use simplelog::{ConfigBuilder, Level, Color};

fn main() {

    let args = Args::parse();

    let sl_config = ConfigBuilder::new()
        .set_level_color(Level::Error, Some(Color::Magenta))
        .set_level_color(Level::Info, Some(Color::Blue))
        .set_level_color(Level::Warn, Some(Color::Yellow))
        .build();
    
    let log_filter =  match (args.verbose, args.debug) {
        (true, true) => simplelog::LevelFilter::Trace,
        (false, true) => simplelog::LevelFilter::Debug,
        (true, false) => simplelog::LevelFilter::Info,
        (false, false) => simplelog::LevelFilter::Off,
    };

    let _ = simplelog::TermLogger::init(log_filter,
            sl_config,
            simplelog::TerminalMode::Stderr,
            simplelog::ColorChoice::Always);

    // user params
    let params_target_workdir = &args.workdir;
    let params_tasks_to_delete = match args.tasks {
        Some(tasks) => tasks,
        None => vec!["".to_string()], // is this dangerous? maybe a task name can be empty?
    };
    let params_limit_lines: usize = args.limit_lines;

    // internal params
    let params_target_filename = ".command.run";
    let params_restring = r"### name:\s*'?([^\s']+)";
    
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
}


mod utils;

mod task;

use task::workflow_task;

mod run;
use run::workflow_run;

use clap::Parser;
mod cli;
use cli::{Cli, Commands};

use log::info;
use simplelog;
use simplelog::{ConfigBuilder, Level, Color};

fn main() {

    let cli = Cli::parse();

    let sl_config = ConfigBuilder::new()
        .set_level_color(Level::Error, Some(Color::Magenta))
        .set_level_color(Level::Info, Some(Color::Blue))
        .set_level_color(Level::Warn, Some(Color::Yellow))
        .build();
    
    let log_filter =  match (cli.verbose, cli.debug) {
        (true, true) => simplelog::LevelFilter::Trace,
        (false, true) => simplelog::LevelFilter::Debug,
        (true, false) => simplelog::LevelFilter::Info,
        (false, false) => simplelog::LevelFilter::Off,
    };

    let _ = simplelog::TermLogger::init(log_filter,
            sl_config,
            simplelog::TerminalMode::Stderr,
            simplelog::ColorChoice::Always);
    
    match &cli.command {
        Commands::Clean {
            logfile,
            force
        } => {
            // internal params, run mode
            let params_cached = r"nextflow.processor.TaskProcessor - \[(\S{2}/[0-9a-f]+)\] Cached process > (\S+)";
            let params_submitted = r"INFO  nextflow.Session - \[(\S{2}/[0-9a-f]+)\] Submitted process > (\S+)";
            let params_runname = r"DEBUG nextflow.Session - Run name: ([a-z]+_[a-z]+)";
            let params_wkdirre = r"DEBUG nextflow.Session - Work-dir: (\S+)";
            info!("Running clean with rundir: {}", logfile);
            let _ = workflow_run(logfile,
                            params_cached,
                            params_submitted,
                            params_runname,
                            params_wkdirre,
                            force);
        },
        Commands::Task {
            workdir,
            tasks,
            limit_lines
        } => {
            // user params, task mode
            let params_target_workdir = workdir;
            let params_tasks_to_delete = match tasks {
                Some(tasks) => tasks.clone(),
                None => vec!["".to_string()], // is this dangerous? maybe a task name can be empty?
            };
            let params_limit_lines: usize = limit_lines.clone();

            // internal params, task mode
            let params_target_filename = ".command.run";
            let params_restring = r"### name:\s*'?([^\s']+)";

            info!("Running task with workdir: {}", params_target_workdir);
            let _ = workflow_task(params_restring,
                            params_target_workdir,
                            params_target_filename,
                            params_limit_lines,
                            params_tasks_to_delete);
        }
    }
}

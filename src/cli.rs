
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "plunger")]
#[command(version = "0.2.0")]
#[command(about = "A tooling program to interact with nextflow pipelines", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    /// Debug output
    #[arg(short, long)]
    pub debug: bool,
    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    Task {
        /// Workdir to parse
        #[arg(short = 'w', long, default_value = "./work")]
        workdir: String,
        /// Tasks that will be deleted
        #[arg(short = 't', long, value_delimiter = ' ')]
        tasks: Option<Vec<String>>,
        /// Maximum lines to check in each .command.run file
        #[arg(short = 'l', long, default_value_t = 30)]
        limit_lines: usize,
    },
    Clean {
        /// nextflow run dir (where the pipeline was run)
        #[arg(short = 'r', long, default_value = ".nextflow.log")]
        logfile: String,
        /// actually delete the directories
        #[arg(short = 'f', long)]
        force: bool,
    },
}

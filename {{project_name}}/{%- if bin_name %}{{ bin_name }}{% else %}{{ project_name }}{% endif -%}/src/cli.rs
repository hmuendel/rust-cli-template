/// The cli module defines all subcommands and sets up the cli parser
///
/// Additional commands can be added via the `Commands` enum
///
use clap::Args;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, Parser, Subcommand, ValueHint,
};
#[cfg(feature = "completion")]
use clap_complete::Shell;
use clap_verbosity_flag::LogLevel;
use clap_verbosity_flag::Verbosity;
use std::path::PathBuf;
use tracing_log::log::Level;

/// `Cli` is the main struct for the cli parser, it contains the gloabl flags
/// and the `Commands` enum with all subcommands
#[derive(Parser)]
#[clap(
    name = crate_name!(),
    author = crate_authors!("\n"),
    version = crate_version!(),
    about = crate_description!(),
    long_about = "Use the Rabin-Miller algorithms to search for possible prime numbers within a range of numbers",
    arg_required_else_help = true
)]
pub struct Cli {
    /// Sets a custom config file
    #[clap(short, long,  value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub config: Option<PathBuf>,
    /// Print build information
    #[clap(long)]
    pub build_info: bool,
    /// The main configuration params live here, some of them can be set as global flags
    #[clap(flatten)]
    pub cfg: Cfg,
    /// The subcommand to execute
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

/// The commands enum contains all subcommands and their respective arguments
#[derive(Subcommand)]
#[clap()]
pub enum Commands {
    #[cfg(feature = "completion")]
    /// Generates completion scripts for the specified shell
    Completion {
        #[clap(value_parser, action)]
        shell: Shell,
    },
    /// Finds prime candidates within a range of numbers
    #[clap(name = "find", value_parser, action)]
    FindPrimesCandidates {
        #[clap(value_name = "LOWER")]
        lower_bound: u32,
        #[clap(value_name = "UPPER")]
        upper_bound: u32,
        #[clap(flatten)]
        config: PrimeCfg,
    },
}

/// This structs contains the global configuration for the application
/// it is merged from the config file, the environment and the command line
/// arguments
#[derive(Args, Debug)]
#[clap()]
pub struct Cfg {
    /// Define the verbosity of the application
    #[clap(flatten)]
    pub verbosity: Verbosity<CustomLevel>,
}

#[derive(Args, Debug)]
pub struct PrimeCfg {
    /// The number of threads to use for the prime candidate search
    #[clap(
        short = 't',
        long,
        value_name = "THREADS",
        required = false,
        default_value = "4"
    )]
    pub number_of_threads: usize,
    /// The iterations of the Rabin-Miller algorithm loop, the higher the
    /// number, the more accurate the result
    #[clap(
        short = 'n',
        long,
        value_name = "NUMBER",
        required = false,
        default_value = "100"
    )]
    pub number_of_iterations: usize,
    /// A list of know primes which will be checked before running the
    /// Rabin-Miller algorithm
    #[clap(short, long)]
    pub known_primes: Vec<u32>,
    /// The separator with which the resulting numbers are separated
    #[clap(short, long, required = false, default_value = " ")]
    pub separator: String,
}

/// This custom log is used to have control over help messages
#[derive(Debug)]
pub struct CustomLevel;

/// Implementing the `LogLevel` trait to customize the help mesages
impl LogLevel for CustomLevel {
    fn default() -> Option<Level> {
        Some(Level::Error)
    }
    fn quiet_help() -> Option<&'static str> {
        Some("Quiet, suppress all logging")
    }
    fn quiet_long_help() -> Option<&'static str> {
        Some("Quiet, suppress all logging")
    }
    fn verbose_help() -> Option<&'static str> {
        Some("Verbosity level, provide multiple times to increase verbosity")
    }
    fn verbose_long_help() -> Option<&'static str> {
        Some("Verbosity level, provide multiple times to increase verbosity")
    }
}

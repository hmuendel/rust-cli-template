use clap::Args;
/// The cli module defines all subcommands and sets up the cli parser
///
/// Additional commands can be added via the `Commands` enum
///
use clap::{
    crate_authors, crate_description, crate_name, crate_version, Parser, Subcommand, ValueHint,
};
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
    long_about = "{{long_description}}",
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
    /// Generates completion scripts for the specified shell
    Completion {
        #[clap(value_parser, action)]
        shell: Shell,
    },
}

/// This structs contains the global configuration for the application
/// it is merged from the config file, the environment and the command line
/// arguments
#[derive(Args, Debug)]
pub struct Cfg {
    /// Define the verbosity of the application
    #[clap(flatten)]
    pub verbosity: Verbosity<CustomLevel>,
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

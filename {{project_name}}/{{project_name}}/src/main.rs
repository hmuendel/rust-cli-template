//! The {{project_name}} command line utility
//!
//! It contains the following subcommands:
//! * `help`: displays a help message
//! * `completion`: generates completion scripts for the specified shell
//!
//! Example usage:
//! ```bash
//! {{project_name}} help completion
//! ```
//! Example output:
//! ```text
//! {{project_name}}-completion
//! Generates completion scripts for the specified shell
//!
//! USAGE:
//!     {{project_name}} completion [OPTIONS] <SHELL>
//!
//! ARGS:
//!     <SHELL>
//!             [possible values: bash, elvish, fish, powershell, zsh]
//!
//! OPTIONS:
//!     -h, --help
//!             Print help information
//!
//!     -q, --quiet
//!             Quiet, suppress all logging
//!
//!     -v, --verbose
//!             Verbosity level, provide multiple times to increase verbosity
//!
//! ```

use clap::{CommandFactory, Parser};
use clap_complete::generate;
use shadow_rs::shadow;
use std::io;

mod cli;
use cli::{Cfg, Cli, Commands};

mod logging;
shadow!(build);
/// The main entrypoint of the cli application. It parses the command line
/// flags and then calls the appropriate subcommand choosen by the `Commands`
/// enum.
fn main() {
    let cli = Cli::parse();
    logging::setup_logging(&cli.cfg);
    if cli.build_info {
        print_build_info();
        return;
    }
    match &cli.command {
        Some(Commands::Completion { shell }) => {
            let mut cmd = Cli::command();
            let cmd_name = cmd.get_name().to_string();
            generate(*shell, &mut cmd, cmd_name, &mut io::stdout());
        }
        None => return,
    }
}
/// Prints the build information gathered at compile time.
fn print_build_info() {
    println!("project_name:         {}", build::PROJECT_NAME);
    println!("pkg_version:          {}", build::PKG_VERSION);

    println!("\n#git");
    println!("commit_id:            {}", build::COMMIT_HASH);
    println!("tag:                  {}", build::TAG);
    println!("branch:               {}", build::BRANCH);
    println!("commit_date:          {}", build::COMMIT_DATE);
    println!("commit_author:        {}", build::COMMIT_AUTHOR);
    println!("commit_email:         {}", build::COMMIT_EMAIL);

    println!("\n#build");
    println!("build_os:             {}", build::BUILD_OS);
    println!("rust_version:         {}", build::RUST_VERSION);
    println!("rust_channel:         {}", build::RUST_CHANNEL);
    println!("cargo_version:        {}", build::CARGO_VERSION);
    println!("build_time:           {}", build::BUILD_TIME);
    println!("build_rust_channel:   {}", build::BUILD_RUST_CHANNEL);
}

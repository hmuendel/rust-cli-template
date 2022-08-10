{% if bin_name -%}
//! The {{ bin_name }} command line utility
{%- else -%}
//! The {{ project_name }} command line utility
{%- endif %}
//!
//! It contains the following subcommands:
//! * `help`: displays a help message
//! * `completion`: generates completion scripts for the specified shell
//!
//! Example usage:
//! ```bash
{% if bin_name -%}
//! {{bin_name}} help completion
{%- else -%}
//! {{project_name}} help completion
{%- endif %}
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

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#[cfg(feature = "completion")]
use clap::CommandFactory;

use clap::Parser;
#[cfg(feature = "completion")]
use clap_complete::generate;
{% if lib_name -%}
use {{ lib_name }};
{%- else -%}
use {{ project_name }}_lib;
{%- endif %}
use shadow_rs::shadow;
#[cfg(feature = "completion")]
use std::io;
use tracing::{info, instrument, trace, warn};
mod cli;
use cli::{Cfg, Cli, Commands};
mod logging;
use logging::setup_logging;

// This pulls in compile time information
shadow!(build);
/// The main entrypoint of the cli application. It parses the command line
/// flags and then calls the appropriate subcommand choosen by the `Commands`
/// enum.
#[instrument(level = "trace")]
fn main() {
    let cli = Cli::parse();
    setup_logging(&cli.cfg);
    trace!("tracing!");

    if cli.build_info {
        print_build_info();
        return;
    }
    match &cli.command {
        #[cfg(feature = "completion")]
        Some(Commands::Completion { shell }) => {
            info!("Generating completion script for {}", shell);
            let mut cmd = Cli::command();
            let cmd_name = cmd.get_name().to_string();
            generate(*shell, &mut cmd, cmd_name, &mut io::stdout());
        }
        Some(Commands::FindPrimesCandidates {
            lower_bound,
            upper_bound,
            config,
        }) => {
            info!(
                "Finding prime candidates in range {}-{}",
                lower_bound, upper_bound
            );

            {% if lib_name -%}
            if let Err(err) = {{ lib_name }}::Config::init(config.into()) {
            {%- else -%}
            if let Err(err) = {{ project_name }}_lib::Config::init(config.into()) {
            {%- endif %}
                eprintln!("{}", err);
                std::process::exit(1);
            }

            {% if lib_name -%}
            let candidates = {{ lib_name }}::find_possible_primes(*lower_bound, *upper_bound);
            {%- else -%}
            let candidates = {{ project_name }}_lib::find_possible_primes(*lower_bound, *upper_bound);
            {%- endif %}
            for candidate in candidates {
                print!("{}{}", candidate, config.separator);
            }
            print!("\n");
        }
        None => return,
    }
}

{% if lib_name -%}
impl Into<{{ lib_name }}::Config> for &cli::PrimeCfg {
{%- else -%}
impl Into<{{ project_name }}_lib::Config> for &cli::PrimeCfg {
{%- endif %}
    {% if lib_name -%}
    fn into(self) -> {{ lib_name }}::Config {
        {{ lib_name }}::Config {
    {%- else -%}
    fn into(self) -> {{ project_name }}_lib::Config {
        {{ project_name }}_lib::Config {
    {%- endif %}
            number_of_threads: self.number_of_threads,
            number_of_iterations: self.number_of_iterations,
            known_primes: self.known_primes.clone(),
        }
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

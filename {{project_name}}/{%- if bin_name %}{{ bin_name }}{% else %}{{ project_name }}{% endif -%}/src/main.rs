{% if bin_name -%}
//! The {{ bin_name }} command line utility
{%- else -%}
//! The {{  project_name | replace(from="-", to="_") }} command line utility
{%- endif %}
//!
//! It contains the following subcommands:
//! * `help`: displays a help message
//! * `completion`: generates completion scripts for the specified shell
{% if example_lib %}
//! * `find`: finds prime candidates with the Rabin-Miller algorithm
{%- endif %}
//!
//! Example usage:
//! ```bash
{% if bin_name -%}
//! {{bin_name}} help help
{%- else -%}
//! {{ project_name | replace(from="-", to="_")}} help help
{%- endif %}
//! ```
//! Example output:
{% if bin_name -%}
//! {{bin_name}}-help
{%- else -%}
//! {{ project_name | replace(from="-", to="_")}}-help
{%- endif %}
//! Print this message or the help of the given subcommand(s)
//! 
//! USAGE:
//!     cli help [OPTIONS] [SUBCOMMAND]...
//! 
//! ARGS:
//!     <SUBCOMMAND>...
//!             The subcommand whose help message to display
//! 
//! OPTIONS:
//!     -q, --quiet
//!             Quiet, suppress all logging
//! 
//!     -v, --verbose
//!             Verbosity level, provide multiple times to increase verbosity
//! ```

#![warn(missing_docs)]
#![warn(rustdoc::missing_crate_level_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#[cfg(feature = "completion")]
use clap::CommandFactory;

use clap::Parser;
#[cfg(feature = "completion")]
use clap_complete::generate;
{% if example_lib %}
{% if lib_name -%}
use {{  lib_name | replace(from="-", to="_") }};
{%- else -%}
use {{  project_name | replace(from="-", to="_") }}_lib;
{%- endif %}
{%- endif %}
#[cfg(feature = "build_info")]
use shadow_rs::shadow;
#[cfg(feature = "completion")]
use std::io;
use tracing::{info, instrument, trace, warn};
mod cli;
use cli::{Cfg, Cli, Commands};
mod logging;
use logging::setup_logging;

// This pulls in compile time information
#[cfg(feature = "build_info")]
shadow!(build);
/// The main entrypoint of the cli application. It parses the command line
/// flags and then calls the appropriate subcommand choosen by the `Commands`
/// enum.
#[instrument(level = "trace")]
fn main() {
    let cli = Cli::parse();
    setup_logging(&cli.cfg);
    trace!("tracing!");

    #[cfg(feature = "build_info")]
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
        {% if example_lib -%}
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
            if let Err(err) = {{  lib_name | replace(from="-", to="_") }}::Config::init(config.into()) {
            {%- else -%}
            if let Err(err) = {{  project_name | replace(from="-", to="_") }}_lib::Config::init(config.into()) {
            {%- endif %}
                eprintln!("{}", err);
                std::process::exit(1);
            }

            {% if lib_name -%}
            let candidates = {{  lib_name | replace(from="-", to="_") }}::find_possible_primes(*lower_bound, *upper_bound);
            {%- else -%}
            let candidates = {{  project_name | replace(from="-", to="_") }}_lib::find_possible_primes(*lower_bound, *upper_bound);
            {%- endif %}
            for candidate in candidates {
                print!("{}{}", candidate, config.separator);
            }
            print!("\n");
        }
        {%- endif -%}
        None => return,
    }
}

{% if example_lib -%}
{% if lib_name -%}
impl Into<{{  lib_name | replace(from="-", to="_") }}::Config> for &cli::PrimeCfg {
{%- else -%}
impl Into<{{  project_name | replace(from="-", to="_") }}_lib::Config> for &cli::PrimeCfg {
{%- endif %}
    {% if lib_name -%}
    fn into(self) -> {{  lib_name | replace(from="-", to="_") }}::Config {
        {{  lib_name | replace(from="-", to="_") }}::Config {
    {%- else -%}
    fn into(self) -> {{  project_name | replace(from="-", to="_") }}_lib::Config {
        {{  project_name | replace(from="-", to="_") }}_lib::Config {
    {%- endif %}
            number_of_threads: self.number_of_threads,
            number_of_iterations: self.number_of_iterations,
            known_primes: self.known_primes.clone(),
        }
    }
}
{%- endif -%}
/// Prints the build information gathered at compile time.
#[cfg(feature = "build_info")]
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

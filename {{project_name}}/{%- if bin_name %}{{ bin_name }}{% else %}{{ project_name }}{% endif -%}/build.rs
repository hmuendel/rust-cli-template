use clap::CommandFactory;
use clap_complete::{generate_to, shells};
use clap_mangen::Man;
use std::fs;
{% if autotag %}
use std::process;
{% endif %}

// the cli source file is included for generation purposes
include!("src/cli.rs");
#[cfg(feature = "build_info")]
fn main() -> shadow_rs::SdResult<()> {
{% if autotag %}
    // tagging the git repo with the version from cargo if the tag doesn't
    // already exist
    if !get_git_tags().contains(&env!("CARGO_PKG_VERSION").to_string()) {
        tag_git_repo();
    }
{% endif %}
    //parsing the cli for generation tasks
    let cli = Cli::command();
    // generating the man pages in a folder in the manifest directory
    create_man_pages(cli.clone());
    // generating the completion functions in a folder in the manifest directory
    create_shell_completions(cli);
    // adding compile time information to the compiled binary
    shadow_rs::new()
}

#[cfg(not(feature = "build_info"))]
fn main() {
{% if autotag %}
    // tagging the git repo with the version from cargo if the tag doesn't
    // already exist
    if !get_git_tags().contains(&env!("CARGO_PKG_VERSION").to_string()) {
        tag_git_repo();
    }
{% endif %}
    //parsing the cli for generation tasks
    let cli = Cli::command();
    // generating the man pages in a folder in the manifest directory
    create_man_pages(cli.clone());
    // generating the completion functions in a folder in the manifest directory
    create_shell_completions(cli);
}

{% if autotag %}
// tagging the git repo with the version from cargo
fn tag_git_repo() {
    let output = process::Command::new("git")
        .args(&[
            "tag",
            "-s",
            "-m",
            concat!("new version: ", env!("CARGO_PKG_VERSION")),
            env!("CARGO_PKG_VERSION"),
        ])
        .output()
        .expect("Failed to execute git tag");
    let git_tag = String::from_utf8_lossy(&output.stdout).trim().to_string();
    println!("{}", git_tag);
}

// get the git tags of the current repo from the git command line
fn get_git_tags() -> Vec<String> {
    let output = process::Command::new("git")
        .args(&["tag", "-l"])
        .output()
        .expect("Failed to execute git rev-parse");
    let git_tags = String::from_utf8_lossy(&output.stdout).trim().to_string();
    git_tags.split("\n").map(|s| s.to_string()).collect()
}
{% endif %}

/// renders the the manpage for the given command
fn render_manpage_for_command(
    dir_name: &mut PathBuf,
    parent: Option<&str>,
    command: clap::Command,
) {
    let mut file_name = String::new();
    if let Some(parent) = parent {
        file_name.push_str(parent);
        file_name.push_str("-");
    }
    file_name.push_str(command.get_name());
    file_name.push_str(".1");
    dir_name.push(file_name);
    let mut man_output_file = fs::File::create(&dir_name).expect("Failed to create man page file");
    Man::new(command)
        .render(&mut man_output_file)
        .expect("Failed to generate man page for subcommand");
    dir_name.pop();
}

fn create_man_pages(cli: clap::Command) {
    let cli_name = cli.get_name().to_string();
    let mut man_dir_path = PathBuf::new();
    man_dir_path.push(env!("CARGO_MANIFEST_DIR"));
    man_dir_path.push("man");
    fs::create_dir_all(&man_dir_path).expect("Failed to create directory");
    cli.get_subcommands().for_each(|c| {
        render_manpage_for_command(&mut man_dir_path, Some(&cli_name), c.clone());
    });
    render_manpage_for_command(&mut man_dir_path, None, cli);
}

fn create_shell_completions(mut cli: clap::Command) {
    let cli_name = cli.get_name().to_string();
    let mut completion_dir_path = PathBuf::new();
    completion_dir_path.push(env!("CARGO_MANIFEST_DIR"));
    completion_dir_path.push("completion");
    fs::create_dir_all(&completion_dir_path).expect("Failed to create directory");
    generate_to(shells::Bash, &mut cli, &cli_name, &completion_dir_path)
        .expect("Failed to generate completion");
    generate_to(shells::Elvish, &mut cli, &cli_name, &completion_dir_path)
        .expect("Failed to generate completion");
    generate_to(shells::Fish, &mut cli, &cli_name, &completion_dir_path)
        .expect("Failed to generate completion");
    generate_to(
        shells::PowerShell,
        &mut cli,
        &cli_name,
        &completion_dir_path,
    )
    .expect("Failed to generate completion");
    generate_to(shells::Zsh, &mut cli, &cli_name, &completion_dir_path)
        .expect("Failed to generate completion");
}

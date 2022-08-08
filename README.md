# rust-cli-template
A very opinionated [kickstart](https://github.com/Keats/kickstart) template for 
creating a rust cli and library. The template tries to build an ergonomic, 
feature packed cli while also encouraging good practices like testing, 
benchmarking and documentation. 

To get started you need to have kickstart installed like this:
```bash
cargo install kickstart
```

then you can apply this template like this:

```bash
kickstart https://github.com/hmuendel/rust-cli-template.git
```

This will open the dialog for project creation. Afterwards a new folder 
with the specified project name should exist in your current working 
directory.

Now you can setup git for the new project. The project should be compile- and
testable and you can go ahead and modify it to your needs.

## Folder layout
An example folder layout for a project called `prime` and after the build script
generated it's outputs
```
├── Cargo.toml
├── LICENSE
├── prime
│   ├── build.rs
│   ├── Cargo.toml
│   ├── completion
│   │   ├── _prime
│   │   ├── prime.bash
│   │   ├── prime.elv
│   │   ├── prime.fish
│   │   └── _prime.ps1
│   ├── LICENSE
│   ├── man
│   │   ├── prime.1
│   │   └── prime-completion.1
│   └── src
│       ├── cli.rs
│       ├── logging.rs
│       └── main.rs
├── prime-lib
│   ├── Cargo.toml
│   ├── LICENSE
│   └── src
│       └── lib.rs
└── README.md
```

## Build script
The build script generates man documentation as well as the completion scripts
for shell completion. If the autotag option was enabled, it also tags the 
current git branch with the version in the Cargo.toml as long as this tag not
already exists. Making the Cargo.toml the only source of truth for the version.

## Debugging
Currently only a [vimpspector](https://github.com/puremourning/vimspector)
config is included which works with LLDB. LLDB and the vimpspector also need to
be installed. With the current vimpspector config, it tries to guess the correct
name of either the compiled test or binary depending of the file it was started
from and might not work when executed from non source code files.

## README
It is recommended to have all documentation inside the source files and then
generate the README via [cargo_readme](https://crates.io/crates/cargo-readme)

```bash
cargo install cargo-readme
```

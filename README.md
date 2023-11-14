# OpenAI Client CLI
[<img alt="crates.io" src="https://img.shields.io/crates/v/openai-client-cli.svg?color=fe7d37&logo=rust" height="20">](https://crates.io/crates/openai-client-cli)
[<img alt="docs.rs" src="https://docs.rs/openai-client-cli/badge.svg" height="20">](https://docs.rs/openai-client-cli/)
[<img alt="GitHub Actions" src="https://github.com/AsherJingkongChen/openai-client-cli/actions/workflows/main.yml/badge.svg" height="20">](https://github.com/AsherJingkongChen/openai-client-cli/actions/workflows/main.yml)

```yaml
Authors: Asher Jingkong Chen
Description: OpenAI API client CLI
```

## Background
I like to use CLI (e.g. cURL) to fetch OpenAI's REST API but ...

Not super easy to use!
That's why I made this **:)**

## Installation guides
The following guides will help you install the CLI program.

### Install via Cargo (1st option)
[Cargo](https://doc.rust-lang.org/cargo/commands/cargo-install.html) will install and compile the program on your machine.
```shell
cargo install openai-client-cli
```

### Install via Homebrew (2nd option)
*\[WIP\]*

### Install from git and build from source codes (3rd option)
1. Copy the source code with `git` and change the working directory
```shell
git clone --recurse-submodules https://github.com/AsherJingkongChen/openai-client-cli.git
cd openai-client-cli
```

2. [Install](https://www.rust-lang.org/tools/install) Rust toolchain manager: `rustup`
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Install Rust toolchain (at least 1.72.0)
```shell
rustup toolchain install 1.72.0
rustup show
```

4. Build in release mode
```shell
cargo build --release
```
or:
```shell
cargo b -r
```

5. Add the directory `target/release` to environment variable `$PATH`
```shell
export PATH='$PATH:target/release'
```

To add the executable in your environment permanently, you can add the export command to ~/.profile, ~/.bashrc or ~/.zshrc.

The command runs everytime when a shell session is launched.

## Usage
Check the following manual to learn how to work on the CLI.

Manual review: [link](https://github.com/AsherJingkongChen/openai-client-cli/blob/master/docs/manual-help.md)

The command to show manual:
```shell
openai --help
```
or:
```shell
openai -h
```

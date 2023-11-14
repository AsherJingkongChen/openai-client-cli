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

## Installation

### Via Cargo
```shell
cargo install openai-client-cli
```

### Via Homebrew
*\[WIP\]*

### Via git and source code
1. Copy the source code and change the working directory
```shell
git clone --recurse-submodules https://github.com/AsherJingkongChen/openai-client-cli.git
cd openai-client-cli
```
2. [Install](https://www.rust-lang.org/tools/install) Rust toolchain manager: `rustup`
```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
3. Install Rust toolchain (1.70.0)
```shell
rustup toolchain install 1.70.0
```
4. Build in release mode
```shell
cargo build --release
```
```shell
cargo b -r
```
5. Add binary directory to environment variable `$PATH`
```shell
export PATH='$PATH:target/release'
```

To add the executable in your environment permanently, you can add the export command to ~/.profile, ~/.bashrc or ~/.zshrc.

The command runs everytime when a session is launched.

## Usage

### Show manual
How-to:
```shell
openai --help
```
```shell
openai -h
```
Preview: [link](https://github.com/AsherJingkongChen/openai-client-cli/blob/master/docs/manual-help.md)

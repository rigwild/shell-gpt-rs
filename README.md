<div align="center">
	<br>
	<br>
	<h1>ShellGPT</h1>
	<br>

Ask ChatGPT for a shell script, code, or anything, directly from your terminal 🤖🧠👨‍💻

[![Cargo Build and Test](https://github.com/rigwild/shell-gpt-rs/actions/workflows/cargo.yml/badge.svg)](https://github.com/rigwild/shell-gpt-rs/actions/workflows/cargo.yml)
[![Downloads](https://img.shields.io/crates/v/shell-gpt)](https://crates.io/crates/shell-gpt)
[![Downloads](https://img.shields.io/crates/d/shell-gpt)](https://crates.io/crates/shell-gpt)

</div>

## Demo

![Demo video](./demo.gif)

## Install

The binary is named `gpt` when installed, not `shell-gpt`.

### Cargo

```bash
cargo install shell-gpt
gpt --help
```

### From source

```bash
git clone git@github.com:rigwild/shell-gpt-rs.git
cd shell-gpt-rs
cargo install --path .
```

## Usage

On first usage, ShellGPT will prompt you to enter your [OpenAI API key](https://platform.openai.com/account/api-keys).

This key will then be saved to `~/.config/shell-gpt-rs`, encrypted with `ChaCha20-Poly1305`, to be reused later.

### Any question

Ask any question to ChatGPT.

```bash
gpt <any question>
```

### `--script`

Ask ChatGPT to create a script, then run it if you want to. A [pre-prompt](./src/openai.rs#L9) will be included with your request.

```bash
gpt --shell show me the 10 biggest files in the current tree
Asking ChatGPT...

#!/bin/bash
du -ah | sort -rh | head -n 10

Do you want to run this script? (y/N) y

Executing script...

--------------


4.1G    ./target
4.1G    .
3.5G    ./target/debug
2.9G    ./target/debug/deps
588M    ./target/release
488M    ./target/release/deps
387M    ./target/debug/build
241M    ./target/debug/incremental
98M     ./target/release/build
86M     ./target/debug/deps/gpt-54b56efa1b0d0573
```

### `--raw`

Ask ChatGPT to create a script, but do not show a loading spinner or interactive prompt, does not run the script.

This can be used to pipe the script to another command.

```bash
gpt --shell --raw show me the 10 biggest files in the current tree
du -ah | sort -rh | head -n 10
```

### `--clear-config`

Remove local config, including the OpenAI API key at `~/.config/shell-gpt-rs`.

```bash
gpt --clear-config
```

### Help message

```bash
gpt --help
```

```
Ask ChatGPT for a shell script, code, or anything, directly from your terminal 🤖🧠👨‍💻

Usage: gpt <your_question>

Options:
  -s  --shell           Ask ChatGPT for a shell script                                                  [boolean=false]
      --raw             Only output the script, no spinner or interactive prompt                        [boolean=false]
      --clear-config    Remove local config, including the OpenAI API key at `~/.config/shell-gpt-rs`   [boolean]
  -h, --help            Show help                                                                       [boolean]

Examples:
  gpt is the earth flat?
  gpt --shell show the list of files in the current directory with human-readable file size
  gpt -s find the top 10 biggest files in the current directory
  gpt -s find the top 10 biggest files in the current tree recursive
  gpt --raw --shell find the top 10 biggest files in the current tree recursive
  gpt --raw --shell find the top 10 biggest files in the current tree recursive > 10_biggest_files.sh
  gpt --raw tell me a good joke | curl -X POST -d @- https://example.com/api/jokes

shell-gpt-rs - https://github.com/rigwild/shell-gpt-rs
```

## Examples

```bash
gpt How to install a package in Arch Linux?
gpt How to create a Rust crate?
gpt --shell show the list of files in the current directory with human-readable file size
gpt -s find the top 10 biggest files in the current tree recursive

# Save the script to a file
gpt --raw --shell find the top 10 biggest files in the current tree recursive > 10_biggest_files.sh

# Generate a joke then post it with curl
gpt --raw tell me a good joke | curl -X POST -d @- https://example.com/api/jokes
```

## License

The MIT license

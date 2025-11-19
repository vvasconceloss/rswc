# rswc

Minimal clone of the Unix `wc` tool.

## Features

- Line counting
- Byte counting
- Simple subcommand interface using `clap`

## Usage

```bash
rswc list <file>
rswc count <file>
```

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run -- list <file>
cargo run -- count <file>
```

# rswc

Minimal clone of the Unix `wc` tool.

## Features

- Line counting
- Byte counting
- Word counting
- Simple subcommand interface using `clap`

## Usage

```bash
rswc list <file>
rswc count <file>
rswc words <file>
```

## Build

```bash
cargo build --release
```

## Run

```bash
cargo run -- list <file>
cargo run -- count <file>
cargo run -- words <file>
```

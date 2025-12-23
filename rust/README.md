# Rust

Rust is used for low-level programming and CLI apps.

## Installation

Install [`rustup`](https://rust-lang.github.io/rustup/) and select the toolchain for [C](../cpp/README.md) compiler (`gcc` or `msvc`).

[Toolchains](https://rust-lang.github.io/rustup/concepts/toolchains.html)

## Usage

### Initialization

```sh
cargo new <name>
cd <name>
```

#### Dependencies

```sh
cargo add <dep1> <dep2>
cargo add <dep3> <dep4> --dev # dev dependencies
```

### Run

```sh
cargo run
```

### Build

```sh
cargo build
```

## Docs

```sh
cargo doc
```

It outputs in `/target` directory and must be manually copied to `/docs` for GitHub Pages.

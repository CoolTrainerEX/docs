# Rust

Rust is used for low-level programming and CLI apps.

## Installation

Install [`rustup`](https://rust-lang.github.io/rustup/) and select the toolchain for C compiler (`gcc` or `msvc`).

[Toolchains](https://rust-lang.github.io/rustup/concepts/toolchains.html)

## Usage

### Initialization

```ps
cargo new <name>
cd <name>
```

#### Install Dependencies

```ps
cargo add <pkg1> <pkg2>
cargo add <pkg3> <pkg4> --dev # Dev dependencies
```

### Run

```ps
cargo run
```

### Build

```ps
cargo build
```

### Docs

```ps
cargo doc
```

It outputs in `/target` directory and must be manually copied to `/docs` for GitHub Pages.

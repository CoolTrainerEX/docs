# JavaScript

JavaScript is used for frontend web development.

- [NextJS](nextjs/README.md)
- [Tauri](tauri/README.md)

## Installation

Install [Deno](https://deno.com/). For CSS Preprocessor, install [Sass](https://sass-lang.com/).

## Usage

### Initialization

Follow individual initializations, or:

```sh
deno init <name>
```

#### Dependencies

```sh
deno add <dep1> <dep2> --allow-scripts
deno add <dep3> <dep4> --allow-scripts -D # dev dependencies
```

[Dependencies](deps.json)

## Docs

```sh
deno doc
```

## Upgrading

After upgrading, run:

```sh
deno clean
```

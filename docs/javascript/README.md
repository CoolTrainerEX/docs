# JavaScript

JavaScript is used for frontend web development.

- [NextJS](nextjs/README.md)
- [Tauri](tauri/README.md)

## Installation

Install [Bun](https://bun.com/).

## Usage

### Initialization

Follow individual initializations, or:

```sh
bun init <name>
```

#### continued

```sh
bun create @eslint/config
```

#### Dependencies

```sh
bun add <dep1> <dep2>
bun add -d <dep3> <dep4> # dev dependencies
```

[Dependencies](deps.json)

## Docs

```sh
deno doc
```

## Upgrading

After upgrading, run:

```sh
bun update -g
bun pm cache rm
```

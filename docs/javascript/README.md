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
cd <name>
bun create @eslint/config
```

#### Dependencies

```sh
bun add <dep1> <dep2>
bun add -d <dep3> <dep4> # dev dependencies
```

[Dependencies](deps.json)

#### Edit

- [`eslint.config.ts`](edit/eslint.config.ts)
- [`package.json`](edit/package.json)

## Docs

```sh
bunx typedoc --entryPointStrategy Expand src
```

## Upgrading

After upgrading, run:

```sh
bun update -g
bun pm cache rm
```

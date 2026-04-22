# JavaScript

JavaScript is used for frontend web development.

- [NextJS](nextjs/README.md)
- [Tauri](tauri/README.md)

## Installation

Install [Node.js](https://nodejs.org/en) and [pnpm](https://pnpm.io/).

## Usage

### Initialization

Follow individual initializations, or:

```sh
mkdir <name>
cd <name>
npm init
pnpm create @eslint/config
```

#### Dependencies

```sh
pnpm add <dep1> <dep2>
pnpm add -D <dep3> <dep4> # dev dependencies
```

[Dependencies](deps.json)

#### Edit

- [`eslint.config.ts`](edit/eslint.config.ts)
- [`package.json`](edit/package.json)

## Docs

```sh
pnpm typedoc --entryPointStrategy Expand src
```

## Upgrading

After upgrading, run:

```sh
pnpm update -g
pnpm store prune
```

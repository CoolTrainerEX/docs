# Roblox

Lua is used for Roblox game development.

## Installation

Install [Rokit](https://github.com/rojo-rbx/rokit) and [Moonwave](https://eryn.io/moonwave/). [Bun](../../javascript//README.md) is needed for Moonwave.

```sh
bun add -g moonwave
```

## Usage

### Initialization

```sh
mkdir <name>
cd <name>
rokit init
rokit add rojo-rbx/rojo 
rokit add UpliftGames/wally
rokit add rojo-rbx/run-in-roblox
rojo init 
wally init
```

#### Dependencies

Edit [`wally.toml`](edit/wally.toml)

```sh
wally install
```

#### Create

- [`scripts/run-tests.luau`](create/scripts/run-tests.luau)
- [`src/jest.config.luau`](create/src/jest.config.luau)
- [`selene.toml`](create/selene.toml)

#### Edit

- [`default.project.json`](edit/default.project.json)
- [`.gitignore`](edit/.gitignore)

### Run

Run in **Roblox Studio**.

### Test

```sh
run-in-roblox --place <name>.rbxl --script scripts/run-tests.luau
```

## Docs

```sh
moonwave dev # or
moonwave build --publish
```

## Upgrading

```sh
rokit self-update
```

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
rojo init 
wally init
```

#### Dependencies

Edit [`wally.toml`](edit/wally.toml)

```sh
wally install
```

#### Edit

- [`.gitignore`](edit/.gitignore)

### Run

Run in **Roblox Studio**.

## Docs

```sh
moonwave dev # or
moonwave build --publish
```

## Upgrading

```sh
rokit self-update
```

# Roblox

Lua is used for Roblox game development.

## Installation

Install [Rokit](https://github.com/rojo-rbx/rokit) and [Moonwave](https://eryn.io/moonwave/). [JavaScript](../../javascript/README.md) is needed for Moonwave.

```sh
pnpm add -g moonwave
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
rokit add JohnnyMorganz/wally-package-types
rojo init
wally init
```

#### Dependencies

Edit [`wally.toml`](edit/wally.toml)

```sh
wally install
```

#### continued

```sh
rojo sourcemap ./default.project.json --output ./sourcemap.json
wally-package-types --sourcemap ./sourcemap.json ./Packages/ ./DevPackages/ ./ServerPackages/
```

#### Create

- [`scripts/run-tests.luau`](create/scripts/run-tests.luau)
- [`src/jest.config.luau`](create/src/__tests__/jest.config.luau)
- [`src/client/App.luau`](create/src/client/App.luau)
- [`src/stories/App.story.luau`](create/src/stories/App.story.luau)
- [`selene.toml`](create/selene.toml)

#### Edit

- [`src/client/init.client.luau`](edit/src/client/init.client.luau)
- [`src/server/init.server.luau`](edit/src/server/init.server.luau)
- [`default.project.json`](edit/default.project.json)
- [`.gitignore`](edit/.gitignore)

### Run

Run in **Roblox Studio**.

### Test

```sh
run-in-roblox --place ./<name>.rbxl --script ./scripts/run-tests.luau
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

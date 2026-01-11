# Gin

Gin is a backend web framework for servers.

## Installation

Install [Deno](../../javascript//README.md) for [Redocly CLI](https://redocly.com/docs/cli)

```sh
deno install -A -g npm:@redocly/cli
```

## Usage

### Initialization

#### Dependencies

[Dependencies](deps.json)

#### continued

```sh
go tool cobra-cli init -a <username> -l <license> --viper
```

#### Create

- [`internal/api/api.go`](create/internal/api/api.go)
- [`internal/api/cfg.yaml`](create/internal/api/cfg.yaml)
- [`internal/db/model/model.go`](create/internal/db/model/model.go)  

## Docs

```sh
redocly build-docs ./openapi.yml -o ./docs/index.html
```

## Upgrading

After upgrading, run:

```sh
deno install -A -g -f npm:@redocly/cli
```

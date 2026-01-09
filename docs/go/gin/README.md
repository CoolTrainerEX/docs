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

#### Create `internal/api/api.go`

```go
package api

//go:generate go tool oapi-codegen -config ./cfg.yaml ../../openapi.yaml
```

#### Create `internal/api/cfg.yaml`

```yaml
# yaml-language-server: $schema=https://raw.githubusercontent.com/oapi-codegen/oapi-codegen/HEAD/configuration-schema.json
package: api
generate:
  gin-server: true
  models: true
  strict-server: true
output: gen.go
```

#### Create `internal/db/model/model.go`

```go
package model

//go:generate go tool gorm gen -i ./model.go
```

## Docs

```sh
redocly build-docs ./openapi.yml -o ./docs/index.html
```

## Upgrading

After upgrading, run:

```sh
deno install -A -g -f npm:@redocly/cli
```

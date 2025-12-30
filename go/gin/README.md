# Gin

Gin is a web framework for servers.

## Usage

### Initialization

#### Dependencies

[Dependencies](deps.json)

#### continued

```sh
go tool cobra-cli init -a <username> -l <license> --viper
```

#### `internal/api/api.go`

```go
package api

//go:generate go tool oapi-codegen -config cfg.yaml ../../api.yaml
```

#### `internal/api/cfg.yaml`

```yaml
# yaml-language-server: $schema=https://raw.githubusercontent.com/oapi-codegen/oapi-codegen/HEAD/configuration-schema.json
package: api
generate:
  gin-server: true
  models: true
output: gen.go
```

#### `internal/db/model/model.go`

```go
package model

//go:generate go tool gorm gen -i model.go
```

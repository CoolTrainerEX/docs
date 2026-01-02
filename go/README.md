# Go

Go is used for server applications.

- [Gin](gin/README.md)

## Installation

Install [Go](https://go.dev/).

## Usage

### Initialization

```sh
mkdir <name>
cd <name>
go mod init github.com/<username>/<name>
docker init
```

#### Dependencies

```sh
go get -u <dep1> <dep2>
go get -tool <dep3> # dev dependencies
go get -tool <dep4>
```

[Dependencies](deps.json)

#### compose.yaml

Uncomment Postgres image, then:

```yaml
services:
  db:
    volumes:
      - db-data:/var/lib/postgresql # Remove /data
```

### Run

**VS Code** Debugger, or:

```sh
go run ./main.go
```

#### Docker

```sh
docker-compose up
```

End

```sh
docker-compose down
```

### Build

```sh
go build
```

## Docs

```sh
go tool doc2go -out ./docs/ ./...
```

## Directory

### lib

- pkg
  - \<name>
    - \<name>.go
    - \<name>_test.go

### exe

- cmd
  - \<name>
    - main.go

### Overall

- cmd
  - \<exe1>
    - main.go
  - \<exe2>
    - main.go

- internal
  - \<exe3>
    - main.go
  - \<exe4>
    - main.go
  - \<lib1>
    - \<lib1>.go
    - \<lib1>_test.go
  - \<lib2>
    - \<lib2>.go
    - \<lib2>_test.go
- pkg
  - \<lib3>
    - \<lib3>.go
    - \<lib3>_test.go
  - \<lib4>
    - \<lib4>.go
    - \<lib4>_test.go

## Upgrading

After upgrading, run:

```sh
 go clean -cache -testcache -modcache -fuzzcache
```

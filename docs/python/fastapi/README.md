# FastAPI

FastAPI is used for API servers.

## Usage

### Initialization

```sh
docker init
```

#### Dependencies

[Dependencies](deps.json)

#### continued

Run this when there is a change in dependencies.

```sh
uv export --format requirements-txt --no-dev --no-emit-project --output-file requirements.txt
```

#### Edit

- [`compose.yaml`](edit/compose.yaml)
- `.dockerignore` (Copy `.gitignore`)

### Run

**VS Code** Debugger, or:

```sh
uv run fastapi dev
```

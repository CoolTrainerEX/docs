# Python

Python is used for general-purpose programming.

## Installation

Install [`uv`](https://docs.astral.sh/uv/).

## Usage

### Initialization

```sh
uv init <name>
cd <name>
```

#### Dependencies

```sh
uv add <dep1> <dep2>
uv add <dep3> <dep4> --dev # dev dependencies
```

[Dependencies](deps.json)

#### continued

```sh
sphinx-quickstart ./docs/ --ext-autodoc # Follow pyproject.toml version, author, etc.
```

#### Create

- [`docs/.gitignore`](create/docs/.gitignore)

#### Edit

- [`pyproject.toml`](edit/pyproject.toml)
- [`docs/conf.py`](edit/docs/conf.py)
- [`docs/index.rst`](edit/docs/index.rst)

### Run

**VS Code** Debugger, or:

```sh
uv run ./main.py
```

## Docs

```sh
sphinx-apidoc -o ./docs/ ./
./docs/make.bat html
```

Do not need to run `make` if using [Read the Docs](https://app.readthedocs.org/dashboard/)

## Upgrading

After upgrading, run:

```sh
uv tool upgrade --all
uv cache prune
```

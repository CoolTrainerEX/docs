# Python

Python is used for general-purpose programming.

## Installation

Install [`uv`](https://docs.astral.sh/uv/).

## Usage

### Initialization

```ps
uv init <name>
cd <name>
```

#### Install Dependencies

```ps
uv add <pkg1> <pkg2>
uv add <pkg3> <pkg4> --dev # Dev dependencies
```

[Dependencies](deps.json)

```ps
sphinx-quickstart docs --ext-autodoc # Follow pyproject.toml version, author, etc
```

#### `pyproject.toml`

```toml
[tool.ruff.lint]
select = ["D"]

[tool.ruff.lint.pydocstyle]
convention = "google"
```

#### `.gitignore`

```gitignore
docs/_*/
```

#### `docs/conf.py`

```py
import sys
from pathlib import Path

extensions = ["sphinx.ext.autodoc", "sphinx.ext.napoleon"]

sys.path.insert(0, str(Path("..").resolve()))
```

#### `docs/index.rst`

```rst
.. toctree::
   :maxdepth: 2
   :caption: Contents:
   
   modules
```

### Run

VS Code Debugger, or

```ps
uv run .\main.py
```

### Docs

```ps
sphinx-apidoc -o docs .
.\docs\make.bat html
```

Do not need to run `make` if using [Read the Docs](https://app.readthedocs.org/dashboard/)

## Upgrading

After upgrading, run:

```ps
uv tool upgrade --all
uv cache prune
```

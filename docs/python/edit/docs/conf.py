import sys
from pathlib import Path

extensions = ["sphinx.ext.autodoc", "sphinx.ext.napoleon"]

sys.path.insert(0, str(Path("..").resolve()))

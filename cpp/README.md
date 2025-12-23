# C++

C++ is used for low-level programming as an alternative to [Rust](../rust/README.md).

## Installation

Install [`mingw`](https://www.mingw-w64.org/) and [Conan](https://conan.io/). [Python](../python/README.md) is needed to install Conan.

```sh
uv tool install conan
conan profile detect
```

### `profiles/default`

```ini
{% if context == "host" %}
[tool_requires]
cmake/[*]
ninja/[*]

[conf]
tools.cmake.cmaketoolchain:generator=Ninja
tools.env.virtualenv:powershell=powershell.exe
{% endif %}
```

## Usage

### Initialization

```sh
mkdir <name>
cd <name>
conan new cmake_lib -d name=<name> # or
conan new cmake_exe -d name=<name>
```

#### Dependencies

During project initialization, add:

```sh
conan new cmake_lib -d name=<name> -d requires=<dep1>/<version> -d requires=<dep2>/<version> -d tool_requires=<dep3>/<version> -d tool_requires=<dep4>/<version>
```

`tool_requires` is dev dependences.

[Dependencies](deps.json)

[Doxygen](https://www.doxygen.nl/) has a broken build, so use global install instead.

#### continued

```sh
conan install . --build=missing
conan install . -s build_type=Debug --build=missing
doxygen -g
```

#### `.gitignore`

```gitignore
/build
```

#### Doxyfile

```sh
OUTPUT_DIRECTORY       = docs
OPTIMIZE_OUTPUT_FOR_C  = YES
RECURSIVE              = YES
EXCLUDE_PATTERNS       = conanfile.py
HTML_OUTPUT            = .
```

### Run

VS Code Debugger

### Build

VS Code CMake Build

## Docs

```sh
doxygen
```

## Directory

### lib

- include
  - \<name>.hpp
- src
  - \<name>.cpp
- CMakeLists.txt
- conanfile.py

### exe

- src
  - \<name>.cpp (main)
  - \<name>.hpp
- CMakeLists.txt
- conanfile.py

### Overall

- include
  - ...
- src
  - ...
- \<lib1>
- \<lib2>
- \<exe1>
- \<exe2>
- ...

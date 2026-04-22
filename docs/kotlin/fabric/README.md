# Fabric

Fabric is used for Minecraft mod development.

## Usage

### Initialization

Use the [Template Mod Generator](https://fabricmc.net/develop/template/)

Use Kotlin. Package name is `com.<username>`.

Run `vscode` and `genSources` Gradle task using [**VS Code Gradle for Java Extension**](https://marketplace.visualstudio.com/items?itemName=vscjava.vscode-gradle), or:

```sh
./gradlew vscode
./gradlew genSources
```

### Run

**VS Code** Debugger, or:

```sh
./gradlew runClient # or
./gradlew runServer
```

### Build

**GitHub Actions**, or:

```sh
./gradlew build
```

# NextJS

NextJS is used to make frontend websites

## Usage

### Initialization

```sh
deno x npm:create-next-app --example with-vitest <name>
deno install --allow-scripts
```

Might need to run in a subdirectory if the current directory contains `desktop.ini` for customizations.

#### next.config.ts

```ts
const nextConfig: NextConfig = {
  output: "export",
};
```

### Run

```sh
deno task dev
```

### Build

**GitHub Actions**, or:

```sh
deno task build
```

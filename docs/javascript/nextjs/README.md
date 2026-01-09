# NextJS

NextJS is used to make frontend websites.

## Usage

### Initialization

```sh
deno x shadcn init
cd <name>
deno init --npm playwright
deno install --allow-scripts
```

Might need to run in a subdirectory if the current directory contains
`desktop.ini` for customizations.

Can use `vitest` template with `--example with-vitest`, but the template is bad,
so just use [manual setup](https://nextjs.org/docs/app/guides/testing/vitest).

#### Dependencies

[Dependencies](deps.json)

#### Create

- [`vite.config.mts`](create/vite.config.ts)

#### Edit

- [`next.config.ts`](edit/next.config.ts)
- [`package.json`](edit/package.json)

### Run

```sh
deno task dev
```

### Build

**GitHub Actions**, or:

```sh
deno task build
```

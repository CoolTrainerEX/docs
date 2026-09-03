# NextJS

NextJS is used to make frontend and full stack websites.

## Usage

### Initialization

```sh
pnpx shadcn init
cd <name>
pnpm create playwright
docker init
```

Might need to run in a subdirectory if the current directory contains
`desktop.ini` for customizations.

Can use `vitest` template with `--example with-vitest`, but the template is bad,
so just use [manual setup](https://nextjs.org/docs/app/guides/testing/vitest).

#### Dependencies

[Dependencies](deps.json)

#### Create

- [`orval.config.ts`](create/orval.config.ts)
- [`vite.config.mts`](create/vite.config.ts)
- [`app/providers.tsx`](create/app/providers.tsx)
- [`.vscode/launch.json`](create/.vscode/launch.json)

#### Edit

- [`next.config.ts`](edit/next.config.ts)
- [`package.json`](edit/package.json)
- [`app/layout.tsx`](edit/app/layout.tsx)

### Run

**VS Code** Debugger, or:

```sh
pnpm dev
```

### Build

**GitHub Actions**, or:

```sh
pnpm build
```

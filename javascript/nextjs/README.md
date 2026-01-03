# NextJS

NextJS is used to make frontend websites.

## Usage

### Initialization

```sh
deno x shadcn init <name>
cd <name>
deno x create-playwright
deno install --allow-scripts
```

Might need to run in a subdirectory if the current directory contains `desktop.ini` for customizations.

Can use `vitest` template with `--example with-vitest`, but the template is bad, so just use [manual setup](https://nextjs.org/docs/app/guides/testing/vitest).

#### Dependencies

[Dependencies](deps.json)

#### `next.config.ts`

```ts
const nextConfig: NextConfig = {
  output: "export",
};
```

#### `vite.config.mts`

```ts
import { defineConfig } from 'vitest/config'
import react from '@vitejs/plugin-react'
import tsconfigPaths from 'vite-tsconfig-paths'
 
export default defineConfig({
  plugins: [tsconfigPaths(), react()],
  test: {
    environment: 'jsdom',
  },
})
```

#### `package.json`

```json
{
  "scripts": {
    "test": "vitest"
  }
}
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

import { defineConfig } from "orval";

export default defineConfig({
  // HTTP client generation
  petstore: {
    input: {
      target: "./openapi.yaml",
    },
    output: {
      mode: "tags-split",
      client: "react-query",
      target: "src/api/endpoints",
      schemas: "src/api/models",
      mock: true,
    },
  },
  // Zod schema generation
  petstoreZod: {
    input: {
      target: "./openapi.yaml",
    },
    output: {
      mode: "tags-split",
      client: "zod",
      target: "src/api/endpoints",
      fileExtension: ".zod.ts",
    },
  },
});

import eslintPluginPrettierRecommended from "eslint-plugin-prettier/recommended";
import { jsdoc } from "eslint-plugin-jsdoc";

export default defineConfig([
    // Any other config imports go at the top
    eslintPluginPrettierRecommended,
    jsdoc({ config: "flat/recommended" })
]);

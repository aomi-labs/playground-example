import react from "@vitejs/plugin-react";
import { fileURLToPath } from "node:url";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      // The widget lazy-loads Para's stylesheet, an optional peer this page
      // does not install. The dev pre-bundler fails on it without a stand-in.
      "@getpara/react-sdk/styles.css": fileURLToPath(new URL("src/empty.css", import.meta.url)),
    },
  },
});

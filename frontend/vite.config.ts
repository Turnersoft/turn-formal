import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import svgr from "vite-plugin-svgr";
import * as path from "node:path";
import dotenv from "dotenv";
import { viteStaticCopy } from "vite-plugin-static-copy";
import * as sass from "sass-embedded";

export default defineConfig(({ mode }) => {
  dotenv.config({ path: `.env.${mode}` });
  return {
    plugins: [
      react(),
      svgr(),
      viteStaticCopy({
        targets: [
          {
            src: "node_modules/@shoelace-style/shoelace/dist/assets/icons/*.svg",
            dest: "assets/icons/",
          },
        ],
      }),
    ],
    build: {
      rollupOptions: {
        input: {
          main: path.resolve(process.cwd(), "index.html"),
        },
      },
    },
    server: {
      proxy: {
        "/api": {
          target: process.env.VITE_API_URL || "http://localhost:3001",
          changeOrigin: true,
          rewrite: (path: string) => path.replace(/^\/api/, ""),
        },
      },
      fs: {
        allow: ["../", "../subjects"],
      },
    },
    resolve: {
      alias: [
        {
          find: "@shoelace-style/shoelace",
          replacement: "/node_modules/@shoelace-style/shoelace",
        },
        {
          find: "@",
          replacement: path.resolve(process.cwd(), "./src"),
        },
        {
          find: "/subjects",
          replacement: path.resolve(process.cwd(), "../subjects"),
        },
      ],
    },
    optimizeDeps: {
      include: ["@shoelace-style/shoelace"],
    },
    css: {
      preprocessorOptions: {
        scss: {
          additionalData: `@use "@/styles/variables.scss" as *;\n`,
          implementation: sass,
          logger: {
            warn: (message) => {
              if (!message.includes("legacy-js-api")) {
                console.warn(message);
              }
            },
          },
          sassOptions: {
            outputStyle: "expanded",
            charset: false,
          },
        },
      },
    },
  };
});

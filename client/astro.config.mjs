import solidJs from "@astrojs/solid-js";
import { defineConfig } from "astro/config";
import playformCompress from "@playform/compress";

// https://astro.build/config
export default defineConfig({
  integrations: [
    solidJs(),
    playformCompress({
      JavaScript: false,
    }),
  ],
});

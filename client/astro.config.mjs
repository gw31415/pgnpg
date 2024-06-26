import solidJs from "@astrojs/solid-js";
import playformCompress from "@playform/compress";
import { defineConfig } from "astro/config";

// https://astro.build/config
export default defineConfig({
  integrations: [
    solidJs(),
    playformCompress({
      JavaScript: false,
    }),
  ],
});

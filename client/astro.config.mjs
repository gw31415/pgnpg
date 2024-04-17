import solidJs from "@astrojs/solid-js";
import { defineConfig } from "astro/config";

import Default from "astro-compress";

// https://astro.build/config
export default defineConfig({
  integrations: [
    solidJs(),
    Default({
      JavaScript: false,
    }),
  ],
});

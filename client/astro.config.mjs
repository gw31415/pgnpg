import { defineConfig ***REMOVED*** from "astro/config";
import solidJs from "@astrojs/solid-js";

import compress from "astro-compress";

// https://astro.build/config
export default defineConfig({
  integrations: [solidJs(), compress()]
***REMOVED***);
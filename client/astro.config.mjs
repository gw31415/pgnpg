import { defineConfig ***REMOVED*** from "astro/config";
import solidJs from "@astrojs/solid-js";

import Default from "astro-compress";

// https://astro.build/config
export default defineConfig({
  integrations: [solidJs(), Default({
    JavaScript: false,
  ***REMOVED***)]
***REMOVED***);

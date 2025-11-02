// @ts-check
import { defineConfig } from 'astro/config';
import { fileURLToPath } from 'node:url';
import { URL } from 'node:url';
import tailwind from '@astrojs/tailwind';
import astroIcon from 'astro-icon';

// https://astro.build/config
export default defineConfig({
  site: 'https://pmikstacki.github.io/bsharp',
  base: '/bsharp',
  integrations: [tailwind(), astroIcon()],
  vite: {
    resolve: {
      alias: {
        '/scripts': fileURLToPath(new URL('./src/scripts', import.meta.url)),
      },
    },
  },
});

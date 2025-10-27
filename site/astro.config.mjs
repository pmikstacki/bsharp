// @ts-check
import { defineConfig } from 'astro/config';
import tailwind from '@astrojs/tailwind';
import astroIcon from 'astro-icon';

// https://astro.build/config
export default defineConfig({
  site: 'https://pmikstacki.github.io/bsharp',
  base: '/bsharp',
  integrations: [tailwind(), astroIcon()],
});

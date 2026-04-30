import { defineConfig } from 'vite';

const crossOriginHeaders = {
  'Cross-Origin-Opener-Policy': 'same-origin',
  'Cross-Origin-Embedder-Policy': 'require-corp',
};

export default defineConfig({
  base: '/pixel-analyzer/',

  server: {
    fs: {
      allow: ['..'],
    },
    headers: crossOriginHeaders,
  },

  preview: {
    headers: crossOriginHeaders,
  },

  optimizeDeps: {
    exclude: ['pixel-analyzer'],
  },

  build: {
    target: 'es2022',
    minify: 'esbuild',
    assetsInlineLimit: 0,
  },

  worker: {
    format: 'es',
  },
});

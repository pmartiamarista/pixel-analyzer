import { defineConfig } from 'vite';
import wasm from 'vite-plugin-wasm';
import topLevelAwait from 'vite-plugin-top-level-await';

const crossOriginHeaders = {
  'Cross-Origin-Opener-Policy': 'same-origin',
  'Cross-Origin-Embedder-Policy': 'require-corp',
};

export default defineConfig({
  plugins: [wasm(), topLevelAwait()],
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

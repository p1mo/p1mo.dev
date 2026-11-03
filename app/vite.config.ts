import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'
// import { resolve } from 'path'

export default defineConfig({
  publicDir: "public",
  input: {
    "index": "index.html",
    "domains": "domains.html"
  },
  build: {
    emptyOutDir: true,
    copyPublicDir: true,
    outDir: "../temp/app",

    target: 'esnext',
    minify: 'terser',
    cssMinify: true,
    sourcemap: false,

    cssCodeSplit: true,
    reportCompressedSize: true,

    terserOptions: {
      parse: {
        bare_returns: false,
        html5_comments: false,
        shebang: false,
      },
      compress: {
        passes: 4,
      },
      mangle: true,
      format: {
        comments: false,
      },
    },

    modulePreload: {
      polyfill: false,
    },
  },
  plugins: [
    tailwindcss(),
  ],
  resolve: {
    alias: {
      //'ui-library': resolve(import.meta.dirname, 'packages/ui-library'),
    }
  }
})
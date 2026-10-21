import { defineConfig } from 'vite'
import tailwindcss from '@tailwindcss/vite'
// import { resolve } from 'path'

export default defineConfig({
  build: {
    emptyOutDir: true,
    outDir: "../temp/app",
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
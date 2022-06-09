import { resolve } from 'path'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      "@": resolve(__dirname, "src")
    }
  },
  build: {
    lib: {
      entry: resolve(__dirname, "src/main.ts"),
      name: "Margo",
      fileName: (f) => `margo.${f}.js`
    },
    rollupOptions: {
      external: ["vue"],

      output: {
        globals: {
          vue: "Vue"
        }
      }
    }
  }
})

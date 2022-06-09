import { resolve } from 'path'
import { defineConfig } from 'vite'

// https://vitejs.dev/config/
export default defineConfig({
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

import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueDevTools from 'vite-plugin-vue-devtools'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueDevTools(),
  ],
  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },

  server: {
    port: 3000,
    proxy: {
      '/api': {
        target: 'http://localhost:8080',
        changeOrigin: true,
        secure: false,
        rewrite: (path) => path.replace(/^\/api/, '')
      }
    }
  },

  build: {
    outDir: 'dist',
    minify: 'terser',
    sourcemap: false, // desactivate sourcemaps for production

    rollupOptions: {
      output: {
        manualChunks: {
          crypto: ['crypto-js'],  // separate crypto-js into its own chunk for optimize  loading
          vue: ['vue', 'vue-router', 'pinia'],
        }
      }
    }
  }
})

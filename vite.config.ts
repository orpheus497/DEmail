import { defineConfig } from 'vite';
import { sveltekit } from '@sveltejs/kit/vite';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [sveltekit()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  // 3. to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.app/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ['VITE_', 'TAURI_'],

  // Phase 5: Build optimizations
  build: {
    // Optimize chunk splitting
    rollupOptions: {
      output: {
        manualChunks: {
          // Group vendor libraries
          vendor: ['svelte', '@sveltejs/kit'],
          // Separate icon library
          icons: ['lucide-svelte'],
        },
      },
    },
    // Increase chunk size warning limit (500kb)
    chunkSizeWarningLimit: 500,
    // Enable minification
    minify: 'esbuild',
    // Generate sourcemaps for production debugging
    sourcemap: false,
  },

  // Optimize dependencies
  optimizeDeps: {
    include: ['svelte', '@sveltejs/kit'],
  },
});

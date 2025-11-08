import { defineConfig } from 'vitest/config';
import { sveltekit } from '@sveltejs/kit/vite';

export default defineConfig({
  plugins: [sveltekit()],
  test: {
    include: ['tests/frontend/**/*.{test,spec}.{js,ts}'],
    globals: true,
    environment: 'happy-dom',
    setupFiles: ['./tests/frontend/setup.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      exclude: ['node_modules/', 'tests/', '**/*.d.ts', '**/*.config.*', '**/mockData.ts'],
    },
  },
  resolve: {
    alias: {
      $lib: '/src/lib',
    },
  },
});

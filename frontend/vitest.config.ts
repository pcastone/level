import { defineConfig } from 'vitest/config'
import { svelte } from '@sveltejs/vite-plugin-svelte'

export default defineConfig({
  plugins: [svelte({ hot: !process.env.VITEST })],
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}'],
    environment: 'jsdom',
    globals: true,
    setupFiles: ['./src/lib/__tests__/setup.ts'],
    coverage: {
      provider: 'v8',
      reporter: ['text', 'json', 'html'],
      include: ['src/lib/**/*.ts', 'src/routes/**/*.svelte'],
      exclude: ['src/**/*.test.ts', 'src/**/__tests__/**']
    }
  },
  resolve: {
    alias: {
      $lib: '/src/lib',
      $env: '/src/env-mock'
    }
  }
})

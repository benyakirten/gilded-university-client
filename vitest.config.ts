import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    include: ['**/*.{test,spec}.{ts,vue}'],
    watchExclude: ['**/node_modules/**', '**/dist/**', '**/playwright/**', '**.*.config.{ts,js,cjs}'],
    exclude: ['**/node_modules/**', '**/dist/**', '**/playwright/**', '**.*.config.{ts,js,cjs}'],
    globals: true,
    environment: "jsdom"
  },
})
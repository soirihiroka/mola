import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import solidSvg from 'vite-plugin-solid-svg'
import { VitePWA } from 'vite-plugin-pwa'

export default defineConfig({
  plugins: [solidPlugin(), solidSvg(), VitePWA({
    registerType: 'prompt',
    injectRegister: false,

    pwaAssets: {
      disabled: false,
      config: true,
    },

    manifest: {
      name: 'Mocap Tool',
      short_name: 'MocapTool',
      description: 'Tool for motion capture',
      theme_color: '#FFFFFFE0',
      "display_override": ["window-controls-overlay"]
    },

    workbox: {
      globPatterns: ['**/*.{js,css,html,svg,png,ico,task}'],
      cleanupOutdatedCaches: true,
      clientsClaim: true,
      maximumFileSizeToCacheInBytes: 3000000,
      runtimeCaching: [
      //   {
      //   urlPattern: /^https:\/\/storage\.googleapis\.com\/mediapipe-models\/.*\.task$/,
      //   handler: 'CacheFirst',
      //   options: {
      //     cacheName: 'google-storage-cache',
      //     expiration: {
      //       maxEntries: 10,
      //       maxAgeSeconds: 60 * 60 * 24 * 30 // <== 30 days
      //     },
      //     cacheableResponse: {
      //       statuses: [0, 200]
      //     }
      //   },
      // },
      // {
      //   urlPattern: /^https:\/\/cdn\.jsdelivr\.net\/npm\/@mediapipe\/tasks-vision\/wasm.*$/,
      //   handler: 'CacheFirst',
      //   options: {
      //     cacheName: 'mediapipe-wasm',
      //     expiration: {
      //       maxEntries: 10,
      //       maxAgeSeconds: 60 * 60 * 24 * 30, // 30 days
      //     },
      //     cacheableResponse: {
      //       statuses: [0, 200],
      //     },
      //   },
      // },
      ]
    },

    devOptions: {
      enabled: false,
      navigateFallback: 'index.html',
      suppressWarnings: true,
      type: 'module',
    },




  }),],
  server: {
    port: 3000,
    allowedHosts: true,
  },
  build: {
    target: 'esnext',
  },
});

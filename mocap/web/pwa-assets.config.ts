import {
    defineConfig,
} from '@vite-pwa/assets-generator/config'

export default defineConfig({
    headLinkOptions: {
        preset: '2023',
    },
    // preset,
    preset:"minimal-2023",
    images: ['public/favicon.svg'],
})

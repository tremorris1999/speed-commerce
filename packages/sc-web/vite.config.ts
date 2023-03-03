import { defineConfig } from 'vite'
import { qwikVite } from '@builder.io/qwik/optimizer'
import { qwikCity } from '@builder.io/qwik-city/vite'
import tsconfigPaths from 'vite-tsconfig-paths'
import { qwikNxVite } from 'qwik-nx/plugins'

export default defineConfig(() => {
  return {
    plugins: [qwikCity(), qwikVite(), qwikNxVite(), tsconfigPaths()],
    preview: {
      headers: {
        'Cache-Control': 'public, max-age=600'
      }
    }
  }
})

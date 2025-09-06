/// <reference types="vite/client" />

interface ImportMetaEnv {
  readonly VITE_MATOMO_URL: string
  readonly VITE_MATOMO_SITE_ID: string
  readonly VITE_ENABLE_MATOMO: string
}

interface ImportMeta {
  readonly env: ImportMetaEnv
}

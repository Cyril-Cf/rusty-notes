import { loadFonts } from './webfontloader'
import vuetify from './vuetify'
import pinia from '../store'
import router from '../router'
import axios from '@/plugins/axios'
import type { App } from 'vue'
import Vue3Toastify, { type ToastContainerOptions } from 'vue3-toastify';

export function registerPlugins(app: App) {
  loadFonts()
  app
    .use(Vue3Toastify, {
      autoClose: 3000,
    } as ToastContainerOptions)
    .use(vuetify)
    .use(router)
    .use(pinia)
    .use(axios, {
      baseUrl: import.meta.env.VITE_BACK_API_URL,
    })
}

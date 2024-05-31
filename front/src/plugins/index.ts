import { loadFonts } from './webfontloader'
import vuetify from './vuetify'
import pinia from '../store'
import router from '../router'
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
}

import { createApp } from 'vue';
import { createPinia } from 'pinia';
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate';
import App from './App.vue';
import router from './router';
import keycloakService from '@services/keycloak';
import authStorePlugin from './plugins/authStorePlugin';

import './style.css';

const pinia = createPinia();
pinia.use(piniaPluginPersistedstate);

const renderApp = () => {
  const app = createApp(App);
  app.use(pinia);
  app.use(authStorePlugin, { pinia });
  app.use(router);
  app.mount('#app');
};

// renderApp();

// Call keycloak service to init on render
keycloakService.CallInit(renderApp);

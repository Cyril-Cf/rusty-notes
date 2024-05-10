import { useAuthStore } from "@stores/authStore";
import keycloakService from "@services/keycloak";
import setupInterceptors from "@services/tokenInterceptors";

// Setup auth store as a plugin so it can be accessed globally in our FE
const authStorePlugin = {
  install(app, option) {
    const store = useAuthStore(option.pinia);

    // Global store
    app.config.globalProperties.$store = store;

    // Setup token interceptor so every FE API calls will have the access token for BE to verify
    setupInterceptors(store);
  },
};

export default authStorePlugin;

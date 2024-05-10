import Keycloak from 'keycloak-js';

const keycloak = new Keycloak({
  url: import.meta.env.VITE_KEYCLOAK_URL,
  clientId: import.meta.env.VITE_KEYCLOAK_CLIENT_ID,
  realm: import.meta.env.VITE_KEYCLOAK_REALM,
});

let authenticated;
let store = null;

/**
 * Initializes Keycloak, then run callback. This will prompt you to login.
 *
 * @param onAuthenticatedCallback
 */
async function init(onInitCallback) {
  try {
    authenticated = await keycloak.init({ onLoad: "login-required" });
    console.log(authenticated);
    onInitCallback();
  } catch (error) {
    console.error('Keycloak init failed');
    console.error(error);
  }
}

/**
 * Initializes store with Keycloak user data
 */
async function initStore(storeInstance) {
  try {
    store = storeInstance;
    store.initOauth(keycloak);

    // Show alert if user is not authenticated
    if (!authenticated) {
      alert('not authenticated');
    }
  } catch (error) {
    console.error('Keycloak init failed');
    console.error(error);
  }
}

/**
 * Logout user
 */
function logout(url) {
  keycloak.logout({ redirectUri: url });
}

/**
 * Refreshes token
 */
async function refreshToken() {
  try {
    await keycloak.updateToken(300); // 300 secs | 5 mins
    return keycloak;
  } catch (error) {
    console.error('Failed to refresh token');
    console.error(error);
  }
}

const KeycloakService = {
  CallInit: init,
  CallInitStore: initStore,
  CallLogout: logout,
  CallTokenRefresh: refreshToken,
};

export default KeycloakService;

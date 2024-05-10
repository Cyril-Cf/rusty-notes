import Keycloak from "keycloak-js";
import { useAuthStore } from "@/stores/authStore";

const keycloak = new Keycloak({
  url: import.meta.env.VITE_KEYCLOAK_URL,
  clientId: import.meta.env.VITE_KEYCLOAK_CLIENT_ID,
  realm: import.meta.env.VITE_KEYCLOAK_REALM,
});

let store = null;

/**
 * Initializes Keycloak, then run callback. This will prompt you to login.
 *
 * @param onAuthenticatedCallback
 */
async function init(onInitCallback) {
  try {
    // await keycloak.init({
    //   onLoad: "login-required",
    // });
    // await onInitCallback();

    keycloak
      .init({ onLoad: "login-required", checkLoginIframe: false })
      .then((auth) => {
        console.log("after login");
        if (auth) {
          console.log(keycloak);
          // isAuthenticated.value = true
          // const acceptedRoles = _.get(theme.value, 'keycloak.roles', [])
          // if (_.isEmpty(acceptedRoles)) hasAccess.value = true
          // else {
          //   const userRoles = _.get(keycloak, 'realmAccess.roles', [])
          //   if (!_.isEmpty(_.intersection(userRoles, acceptedRoles))) hasAccess.value = true
          //   else $q.dialog({
          //     title: 'Accés refusé',
          //     message: 'Vous n\'êtes pas autorisé à accèder à ce site'
          //   }).onOk(() => {
          //     window.location.href=theme.value.keycloak.fallbackUrl
          //   })
          // }
        }
      });
  } catch (error) {
    console.error("Keycloak init failed");
    console.error(error);
  }
}

function SaveUserToStore() {
  const authenticated = keycloak.authenticated;
  console.log(keycloak);
  if (authenticated) {
    console.log(
      `User is ${authenticated ? "authenticated" : "not authenticated"}`
    );
    const authStore = new useAuthStore();
    authStore.initOauth(keycloak);
  }
}

/**
 * Logout user
 */
function logout() {
  keycloak.logout({ redirectUri: import.meta.env.VITE_APP_URL });
}

/**
 * Refreshes token
 */
async function refreshToken() {
  try {
    await keycloak.updateToken(300); // 300 secs | 5 mins
    return keycloak;
  } catch (error) {
    console.error("Failed to refresh token");
    console.error(error);
  }
}

const KeycloakService = {
  CallInit: init,
  CallLogout: logout,
  CallTokenRefresh: refreshToken,
  CallSaveUserToStore: SaveUserToStore,
  CallKeycloak: keycloak,
};

export default KeycloakService;

<template>
  <div>
    <h1>Bienvenue sur la page d'accueil !</h1>
    <button type="button" @click="handleLogout">
      {{ $store.authenticated ? "Se déconnecter" : "Se connecter" }}
    </button>
  </div>
</template>

<script setup>
import { useAuthStore } from "@/stores/authStore";
import keycloakService from "@services/keycloak";

const authStore = useAuthStore();
const isAuthenticated = authStore.authenticated;
const handleLogout = async () => {
  if (isAuthenticated) {
    await authStore.logout();
  } else {
    await keycloakService.CallInit(() =>
      authStore.initOauth(keycloakService.keycloak)
    );
  }
};
</script>

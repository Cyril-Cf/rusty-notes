<template>
  <h1>User Secure</h1>
</template>

<script lang="ts" setup>
import { useUserStore } from "@/store/userStore";
import { onMounted } from "vue";
import authPromise from "@/plugins/keycloak";
onMounted(async () => {
  authPromise.then(async (auth) => {
    if (auth.isAuthenticated()) {
      const token = await auth.bearerToken();
      const userStore = useUserStore();
      await userStore.fetchUsers(token!);
    }
  });
});
</script>

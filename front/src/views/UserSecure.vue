<template>
  <h1>User Secure</h1>
</template>

<script lang="ts" setup>
import { useUserStore } from "@/store/userStore";
import { onMounted } from "vue";
import authPromise from "@/plugins/keycloak";
import router from "@/router";
onMounted(async () => {
  authPromise.then(async (auth) => {
    if (auth.isAuthenticated()) {
      const userStore = useUserStore();
      let userIsInDB = await userStore.DoesUserExistInDB(auth.userId()!);
      if (!userIsInDB) {
        router.push({ path: "/subscription_more_infos/user_secure" });
      }
    }
  });
});
</script>

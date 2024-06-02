<template>
  <div>
    <v-menu min-width="200px" rounded>
      <template v-slot:activator="{ props }">
        <v-btn icon v-bind="props">
          <v-avatar :image="gravatarImage" size="32"> </v-avatar>
        </v-btn>
      </template>
      <v-card class="px-5">
        <v-card-text>
          <div class="mx-auto text-center">
            <v-avatar class="my-2" :image="gravatarImage" size="48"> </v-avatar>
            <h3>{{ fullname }}</h3>
            <p class="text-caption mt-1">{{ userEmail }}</p>
            <v-divider class="my-3"></v-divider>
            <v-btn color="primary" variant="text" @click="logoutAccount()" prepend-icon="mdi-logout-variant">
              Se déconnecter
            </v-btn>
          </div>
        </v-card-text>
      </v-card>
    </v-menu>
  </div>
</template>

<script lang="ts" setup>
import { computed, onMounted, onUnmounted } from "vue";
import { Md5 } from "ts-md5";
import authPromise from "@/plugins/keycloak";
import { useUserStore } from "@/store/userStore";
import { useNotificationStore } from "@/store/notificationStore";
import { WebSocketMessage } from "@/types/WebSocketMessage";
import { useListStore } from "@/store/listStore";

const userStore = useUserStore();
let ws: WebSocket | null = null;

const fullname = computed(() => {
  if (userStore.currentUser) {
    return `${userStore.currentUser.firstname} ${userStore.currentUser.lastname}`;
  } else {
    return "";
  }
});

const userEmail = computed(() => {
  if (userStore.currentUser) {
    return userStore.currentUser.email;
  } else {
    return "";
  }
});

const gravatarImage = computed(() => {
  if (userStore.currentUser) {
    return "https://www.gravatar.com/avatar/" +
      Md5.hashStr(userStore.currentUser.email.valueOf()) +
      "?s=48&d=wavatar";
  } else {
    return "https://www.gravatar.com/avatar/" +
      Md5.hashStr("test.test.fr") +
      "?s=48&d=wavatar";
  }
});

const logoutAccount = () =>
  authPromise.then(async (auth) => {
    auth.logout(`${location.origin}`);
    if (ws) {
      ws.close();
    }
  });

onMounted(async () => {
  authPromise.then(async (auth) => {
    if (auth.isAuthenticated()) {
      const userStore = useUserStore();
      const listStore = useListStore();
      const notificationStore = useNotificationStore();
      let userIsInDB = await userStore.DoesUserExistInDB(auth.userId()!);
      if (userIsInDB && userStore.currentUser) {
        let userId = userStore.currentUser.id;
        ws = new WebSocket(`${import.meta.env.VITE_BACK_WS_URL}${userId}`);
        ws.onmessage = async (event) => {
          await notificationStore.fetchNotifications(userId);
          let message: WebSocketMessage = event.data;
          if (message == WebSocketMessage.RefreshFriendships) {
            await userStore.getFriendships(userStore.currentUser!.id);
          } else if (message == WebSocketMessage.RefreshSelectedList) {
            console.log('refresh selected list');
            await listStore.fetchOne(listStore.selectedList!.id);
          } else if (message == WebSocketMessage.RefreshLists) {
            await listStore.fetchLists(userStore.currentUser!.id);
          }
        };
      }
    }
  });
});

onUnmounted(() => {
  if (ws) {
    ws.close();
  }
});
</script>

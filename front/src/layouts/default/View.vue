<template>
  <v-layout class="rounded rounded-md">
    <v-app-bar color="primary" density="compact" class="px-3">
      <template v-slot:prepend>
        <v-app-bar-nav-icon @click="goToHomePage">
          <v-icon>
            mdi-list-box
          </v-icon>
        </v-app-bar-nav-icon>
      </template>
      <template v-slot:title>
        <v-btn @click="goToHomePage">
          Rusty Notes
        </v-btn>
      </template>
      <template v-slot:append>
        <Header />
        <ProfileMenu></ProfileMenu>
      </template>
    </v-app-bar>
    <v-main>
      <router-view />
    </v-main>
  </v-layout>
</template>

<script lang="ts" setup>
import ProfileMenu from "@/components/profile-menu/ProfileMenu.vue";
import Header from '@/components/header/Header.vue'
import router from "@/router";
const goToHomePage = () => {
  router.push({ path: "/" });
};
</script>

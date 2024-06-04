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
      <template v-if="mdAndUp" v-slot:title>
        <v-btn @click="goToHomePage">
          Rusty Notes
        </v-btn>
      </template>
      <template v-slot:append>
        <Header v-if="mdAndUp" />
        <ProfileMenu></ProfileMenu>
        <v-btn v-if="!mdAndUp" icon="mdi-dots-vertical" @click="drawer = true"></v-btn>
      </template>
    </v-app-bar>
    <v-navigation-drawer v-model="drawer" location="right" temporary>
      <HeaderDrawer />
    </v-navigation-drawer>
    <v-main>
      <router-view />
    </v-main>
  </v-layout>
</template>

<script lang="ts" setup>
import ProfileMenu from "@/components/profile-menu/ProfileMenu.vue";
import Header from '@/components/header/Header.vue';
import HeaderDrawer from '@/components/header/HeaderDrawer.vue';
import router from "@/router";
import { useDisplay } from 'vuetify'
import { ref } from "vue";
const { mdAndUp } = useDisplay()
const drawer = ref(false);
const goToHomePage = () => {
  router.push({ path: "/" });
};
</script>

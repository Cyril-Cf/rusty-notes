<template>
  <v-layout class="rounded rounded-md">
    <v-app-bar class="px-3">
      <v-app-bar-nav-icon variant="text" @click.stop="drawer = !drawer">
      </v-app-bar-nav-icon>
      <v-toolbar-title>Rusty Notes</v-toolbar-title>
      <v-spacer></v-spacer>
      <ProfileMenu></ProfileMenu>
    </v-app-bar>

    <v-navigation-drawer v-model="drawer">
      <v-list nav>
        <v-list-item prepend-icon="mdi-home-outline" title="Home" value="home" to="/"></v-list-item>
        <v-list-item prepend-icon="mdi-view-list-outline" title="Mes listes" value="my_lists"
          to="/my_lists"></v-list-item>
        <v-list-item prepend-icon="mdi-account-heart-outline" title="Mes amis" value="my_friends"
          to="/my_friends"></v-list-item>
      </v-list>
    </v-navigation-drawer>

    <v-main>
      <router-view />
    </v-main>
  </v-layout>
</template>

<script lang="ts" setup>
import { ref } from "vue";
import ProfileMenu from "@/components/profile-menu/ProfileMenu.vue";

const drawer = ref(true);
</script>

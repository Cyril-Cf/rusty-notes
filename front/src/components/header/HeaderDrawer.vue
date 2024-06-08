<template>
    <v-list v-if="isAuth">
        <v-list-item v-for="link in links" :key="link.name">
            <v-btn :to="link.to" class="mx-1" color="primary" rounded="sm" variant="text">
                {{ link.name }}
            </v-btn>
        </v-list-item>
    </v-list>
    <v-list v-else>
        <v-list-item>
            <v-btn :href="registerUrl.valueOf()" target="_blank" class="mx-0 mx-lg-1" color="primary" rounded="sm"
                variant="text">
                S'inscrire
            </v-btn>
        </v-list-item>
        <v-list-item>
            <v-btn :href="loginUrl.valueOf()" target="_blank" class="mx-0 mx-lg-1" color="primary" rounded="sm"
                variant="text">
                Se connecter
            </v-btn>
        </v-list-item>
    </v-list>
</template>

<script lang="ts" setup>
import authPromise from "@/plugins/keycloak";
import { ref } from "vue";
import { useDisplay } from 'vuetify'
const { mdAndUp } = useDisplay()
const isAuth = ref(false);
const loginUrl = ref<String>('');
const registerUrl = ref<String>('');
authPromise.then((auth) => {
    isAuth.value = auth.isAuthenticated();
    loginUrl.value = auth.getLoginUrl();
    registerUrl.value = auth.getRegisterUrl();
});
const links = [
    { name: 'Home', to: "/" },
    { name: 'Mes notes', to: "/my_notes" },
    { name: 'Mes amis', to: "/my_friends" },
]
</script>
<template>
    <v-container style="min-width: 40vw">
        <v-row class="mt-0" justify="center" align="center" no-gutters>
            <v-col v-if="isAuth" class="text-center" cols="12">
                <v-btn v-for="link in links" :key="link.name" :to="link.to" class="mx-1" color="white" rounded="sm"
                    variant="text">
                    {{ link.name }}
                </v-btn>
            </v-col>
            <v-col v-else class="text-center" cols="12">
                <v-btn :href="registerUrl.valueOf()" target="_blank" class="mx-0 mx-lg-1" color="white" rounded="sm"
                    variant="text">
                    S'inscrire
                </v-btn>
                <v-btn :href="loginUrl.valueOf()" target="_blank" class="mx-0 mx-lg-1" color="white" rounded="sm"
                    variant="text">
                    Se connecter
                </v-btn>
            </v-col>
        </v-row>
    </v-container>
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
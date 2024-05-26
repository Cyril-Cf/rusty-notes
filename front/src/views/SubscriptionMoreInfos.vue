<template>
    <div>
        <h1>J'ai besoin de quelques infos avant de se lancer !</h1>
        <v-form v-model="valid" @submit.prevent="submitForm">
            <v-container>
                <v-row>
                    <v-col cols="12" md="4">
                        <v-text-field v-model="firstname" :counter="10" :rules="nameRules" label="First name"
                            hide-details required></v-text-field>
                    </v-col>

                    <v-col cols="12" md="4">
                        <v-text-field v-model="lastname" :counter="10" :rules="nameRules" label="Last name" hide-details
                            required></v-text-field>
                    </v-col>

                    <v-col cols="12" md="4">
                        <v-text-field v-model="email" :rules="emailRules" label="E-mail" hide-details
                            required></v-text-field>
                    </v-col>
                </v-row>
                <v-row>
                    <v-col cols="4">
                        <v-btn type="submit" color="primary" :disabled="!valid">Et voilà !</v-btn>
                    </v-col>
                </v-row>
            </v-container>
        </v-form>
    </div>
</template>

<script setup>
import { ref } from 'vue';
import { useUserStore } from "@/store/userStore";
import authPromise from "@/plugins/keycloak";
import router from "@/router";
import { useRoute } from 'vue-router';

const route = useRoute();
const redirectUri = route.params.redirectUri;
const valid = ref(false);
const firstname = ref('');
const lastname = ref('');
const email = ref('');

const nameRules = [
    value => {
        if (value) return true;
        return 'Name is required.';
    },
    value => {
        if (value?.length <= 10) return true;
        return 'Name must be less than 10 characters.';
    },
];

const emailRules = [
    value => {
        if (value) return true;
        return 'E-mail is required.';
    },
    value => {
        if (/.+@.+\..+/.test(value)) return true;
        return 'E-mail must be valid.';
    },
];

const submitForm = async () => {
    authPromise.then(async (auth) => {
        if (auth.isAuthenticated()) {
            const userStore = useUserStore();
            const input = {
                firstname: firstname.value,
                lastname: lastname.value,
                email: email.value,
                keycloakUuid: auth.userId()
            };
            await userStore.createCurrentUser(input);
            let userIsInDB = await userStore.DoesUserExistInDB(auth.userId());
            if (userIsInDB) {
                router.push(`/${redirectUri}`);
            } else {
                console.log("On a un petit probleme !");
            }
        }
    });
};
</script>
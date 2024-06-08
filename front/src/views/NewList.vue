<template>
    <v-container class="my-5 mx-auto" fluid style="width: 80vw;">
        <v-row no-gutters class="mb-5">
            <ButtonGoBackToMyLists />
        </v-row>
        <v-row>
            <v-col cols="12">
                <h1 class="pb-5">Créer une nouvelle liste</h1>
                <NewList />
            </v-col>
        </v-row>
    </v-container>
</template>

<script lang="ts" setup>
import { onMounted } from 'vue';
import ButtonGoBackToMyLists from '../components/list/common/ButtonGoBackToMyLists.vue'
import NewList from '../components/list/single-list/NewList.vue'
import { useUserStore } from "@/store/userStore";
import authPromise from "@/plugins/keycloak";
import router from "@/router";

onMounted(async () => {
    authPromise.then(async (auth) => {
        if (auth.isAuthenticated()) {
            const userStore = useUserStore();
            let userIsInDB = await userStore.DoesUserExistInDB(auth.userId()!);
            if (!userIsInDB) {
                router.push({ path: "/subscription_more_infos/add_list" });
            }
        }
    });
});
</script>

<style scoped></style>
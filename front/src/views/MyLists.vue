<template>
    <div>
        <h1>Mes listes</h1>
        <v-container>
            <v-row>
                <v-col v-for="(list, index) in listStore.lists" :key="index">
                    <v-card>
                        <v-card-title>{{ list.name }}</v-card-title>
                        <v-card-text>
                            <v-chip v-for="(tag, index) in list.tags" :key="index" class="ma-1" outlined>{{ tag
                                }}</v-chip>
                        </v-card-text>
                        <v-card-actions>
                            <v-btn @click="goToSingleList(index)" color="primary">Détails</v-btn>
                            <v-btn @click="deleteList(index)" color="error">Supprimer</v-btn>
                        </v-card-actions>
                    </v-card>
                </v-col>
            </v-row>
            <v-row>
                <v-col cols="12">
                    <v-btn @click="showAddListDialog" color="primary">Ajouter une liste</v-btn>
                </v-col>
            </v-row>
        </v-container>
    </div>
</template>

<script lang="ts" setup>
import { useListStore } from "@/store/listStore";
import { useUserStore } from "@/store/userStore";
import { onMounted } from "vue";
import authPromise from "@/plugins/keycloak";
import router from "@/router";

const userStore = useUserStore();
const listStore = useListStore();

const showAddListDialog = () => {
    router.push({ path: "/add_list" });
};

const deleteList = async (index: any) => {
    const userId = userStore.currentUser?.id;
    const listId = listStore.lists.at(index)?.id;
    if (userId && listId) {
        await listStore.deleteSelectedList(listId, userId);
    }
}

const goToSingleList = (index: number) => {
    const listId = listStore.lists.at(index)?.id;
    if (listId) {
        router.push({ path: `/single_list/${listId}` });
    }

}

onMounted(async () => {
    authPromise.then(async (auth) => {
        if (auth.isAuthenticated()) {
            const userStore = useUserStore();
            let userIsInDB = await userStore.DoesUserExistInDB(auth.userId()!);
            if (userIsInDB) {
                let userId = userStore.currentUser?.id;
                if (userId) {
                    const listStore = useListStore();
                    listStore.fetchLists(userId);
                }
            } else {
                router.push({ path: "/subscription_more_infos/my_lists" });
            }
        }
    });
});
</script>

<style scoped></style>
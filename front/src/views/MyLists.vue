<template>
    <v-container class="mx-10 my-5 mx-auto" fluid style="width: 80vw;">
        <v-row no-gutters>
            <v-btn @click="pushToNewList" color="primary">Ajouter une liste</v-btn>
        </v-row>
        <v-row no-gutters>
            <v-col cols="12">
                <div class="text-h4 my-10">Mes notes</div>
            </v-col>
            <v-col cols="12" v-for="(list, index) in listStore.ownedLists" :key="index">
                <ListItem :list="list" @deleteListEmit="deleteListOpenModale" @goToSingleListEmit="goToSingleList"
                    @openSettingsEmit="openSettings" />
            </v-col>
        </v-row>
        <v-divider class="my-4" v-if="listStore.sharedListsValidated.length > 0"></v-divider>
        <v-row v-if="listStore.sharedListsValidated.length > 0">
            <v-col cols="12">
                <div class="text-h4 my-10">Mes notes partagées</div>
            </v-col>
            <v-col cols="12" v-for="(list, index) in listStore.sharedListsValidated" :key="index">
                <ListItem :list="list" @goToSingleListEmit="goToSingleList" />
            </v-col>
        </v-row>
        <v-divider class="my-4" v-if="listStore.sharedListToValidate.length > 0"></v-divider>
        <v-row v-if="listStore.sharedListToValidate.length > 0">
            <v-col cols="12">
                <div class="text-h4 my-10">Mes invitations</div>
            </v-col>
            <v-col cols="12" v-for="(list, index) in listStore.sharedListToValidate" :key="index">
                <ListItem :list="list" @acceptInvitationEmit="acceptInvitation"
                    @refuseInvitationEmit="refuseInvitation" />
            </v-col>
        </v-row>
        <v-dialog v-model="settingsDialog" max-width="600px">
            <ListSettingModal @closeSettingsEmit="settingsDialog = false" />
        </v-dialog>
        <v-dialog v-model="removeListDialog" max-width="300px">
            <RemoveListModal @closeRemoveListModalEmit="removeListDialog = false" @RemoveListConfirmEmit="deleteList" />
        </v-dialog>
    </v-container>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import ListItem from '../components/list/list-list/ListItem.vue'
import ListSettingModal from '@/components/list/modal/ListSettingModal.vue';
import RemoveListModal from '../components/list/modal/RemoveListModal.vue'
import { useListStore } from "@/store/listStore";
import { useUserStore } from "@/store/userStore";
import authPromise from "@/plugins/keycloak";
import router from "@/router";
import { List } from '@/types/List';

const userStore = useUserStore();
const listStore = useListStore();
const settingsDialog = ref(false);
const removeListDialog = ref(false);
const listToRemoveId = ref<String | null>(null);

const pushToNewList = () => {
    router.push({ path: "/add_list" });
};

const acceptInvitation = async (listId: String) => {
    if (userStore.currentUser) {
        listStore.acceptListInvitationStore(listId, userStore.currentUser.id)
    }
}

const refuseInvitation = async (listId: String) => {
    if (userStore.currentUser) {
        listStore.refuseListInvitationStore(listId, userStore.currentUser.id)
    }
}

const deleteListOpenModale = (listId: String) => {
    listToRemoveId.value = listId;
    removeListDialog.value = true;
}

const deleteList = async () => {
    const userId = userStore.currentUser?.id;
    if (userId && listToRemoveId.value) {
        await listStore.deleteSelectedList(listToRemoveId.value, userId);
    }
    removeListDialog.value = false
};

const goToSingleList = (listId: string) => {
    router.push({ path: `/single_list/${listId}` });
};

const openSettings = async (list: List) => {
    await listStore.fetchOne(list.id);
    settingsDialog.value = true;
};

onMounted(async () => {
    authPromise.then(async (auth) => {
        if (auth.isAuthenticated()) {
            let userIsInDB = await userStore.DoesUserExistInDB(auth.userId()!);
            if (userIsInDB) {
                let userId = userStore.currentUser?.id;
                if (userId) {
                    listStore.fetchLists(userId);
                    await userStore.getFriendships(userId);
                }
            } else {
                router.push({ path: "/subscription_more_infos/my_notes" });
            }
        }
    });
});
</script>

<style scoped></style>
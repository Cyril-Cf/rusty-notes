<template>
    <v-data-table :headers="headers" :items="userStore.userFriends" fixed-footer width="auto" height="300px"
        items-per-page-text="Item par page" :pageText="'{0}-{1} sur {2}'" no-data-text="Pas encore d'amitiée">
        <template v-slot:top>
            <v-toolbar flat>
                <v-toolbar-title>Mes amis <v-icon color="red-accent-4">mdi-heart-circle</v-icon></v-toolbar-title>
                <v-divider class="mx-4" inset vertical></v-divider>
                <v-spacer></v-spacer>
            </v-toolbar>
        </template>
        <template v-slot:item.friendSince="{ item }">
            {{ formatFriendshipDate(item) }}
        </template>
        <template v-slot:item.action="{ item }">
            <v-icon color="error" size="small" @click="openDeleteModal(item)">
                mdi-delete
            </v-icon>
            <v-dialog v-model="showDeleteModal" max-width="500px">
                <DeleteFriendModal @closeDeleteModalEmit="showDeleteModal = false" @deleteFriendEmit="deleteFriend" />
            </v-dialog>
        </template>
    </v-data-table>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { useUserStore } from "@/store/userStore";
import { User } from '@/types/User';
import DeleteFriendModal from '../modal/DeleteFriendModal.vue';
import { VDataTable } from 'vuetify/labs/VDataTable'
import { formatDate } from '@/service';

const userStore = useUserStore();
const showDeleteModal = ref(false);
const friendToDelete = ref<User | null>(null);

const headers = [
    { title: 'Prénom', key: 'firstname', sortable: false },
    { title: 'Nom', key: 'lastname', sortable: false },
    { title: 'Email', key: 'email', sortable: false },
    { title: 'Amitiée née le', key: 'friendSince', sortable: false },
    { title: 'Action', key: 'action', sortable: false },
];

const formatFriendshipDate = (user: User) => {
    if (user.friendSince) {
        return formatDate(user.friendSince as unknown as string);
    }
    return 'Date non trouvée'
}

const openDeleteModal = (friend: User) => {
    friendToDelete.value = friend;
    showDeleteModal.value = true;
}

const deleteFriend = async () => {
    await userStore.removeFriendship(userStore.currentUser!.id.toString(), friendToDelete.value!.id);
    showDeleteModal.value = false;
};
</script>

<style scoped></style>
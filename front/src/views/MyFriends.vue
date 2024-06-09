<template>
    <v-container class="mx-10 my-5 mx-auto" fluid :style="mdAndUp ? 'width: 80vw' : 'style: 95vw'">
        <v-row no-gutters class="mb-5">
            <v-btn color="primary" @click="showAddFriendModal = true">Ajouter un ami</v-btn>
        </v-row>
        <v-row no-gutters>
            <v-col cols="12">
                <MyFriendsList />
            </v-col>
        </v-row>
        <v-row no-gutters class="mt-10">
            <v-col cols="12">
                <FriendDemandList />
            </v-col>
        </v-row>
        <v-dialog v-model="showAddFriendModal" max-width="500px">
            <AddFriendModal @closeAddFriendModalEmit="showAddFriendModal = false" @addFriendEmit="addFriend" />
        </v-dialog>
    </v-container>
</template>

<script setup>
import { ref, onMounted } from 'vue';
import { useUserStore } from "@/store/userStore";
import authPromise from "@/plugins/keycloak";
import MyFriendsList from '@/components/friend/list-friend/MyFriendsList.vue';
import FriendDemandList from '@/components/friend/list-friend/FriendDemandList.vue';
import AddFriendModal from '@/components/friend/modal/AddFriendModal.vue';
import { useDisplay } from 'vuetify'
const { mdAndUp } = useDisplay()
const userStore = useUserStore();
const showAddFriendModal = ref(false);

const addFriend = async (newFriendEmail) => {
    await userStore.addUserFriendship(userStore.currentUser.id, newFriendEmail);
    showAddFriendModal.value = false;
};

onMounted(async () => {
    authPromise.then(async (auth) => {
        if (auth.isAuthenticated()) {
            const userStore = useUserStore();
            let userId = userStore.currentUser?.id;
            if (userId) {
                await userStore.getFriendships(userId);
            }
        }
    });
});
</script>

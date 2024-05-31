<template>
    <v-container>
        <v-row>
            <v-col cols="12" md="6">
                <h3>Demandes d'amis</h3>
                <v-list>
                    <v-list-item v-for="friendship in friendRequests" :key="friendship.id">
                        <v-list-item-title>{{ getFriendName(friendship) }}</v-list-item-title>
                        <v-list-item-action v-if="isRecipient(friendship)">
                            <v-btn @click="confirmRequest(friendship)">Confirmer</v-btn>
                        </v-list-item-action>
                        <v-list-item-subtitle v-else>En attente de confirmation</v-list-item-subtitle>
                    </v-list-item>
                </v-list>
            </v-col>
            <v-col cols="12" md="6">
                <h3>Amis</h3>
                <v-list>
                    <v-list-item v-for="friend in friends" :key="friend.id">
                        <v-list-item-title>{{ friend.firstname }} {{ friend.lastname }}</v-list-item-title>
                        <v-list-item-action>
                            <v-btn color="red" @click="openDeleteModal(friend)">Supprimer</v-btn>
                        </v-list-item-action>
                    </v-list-item>
                </v-list>
                <v-btn color="primary" @click="openAddFriendModal">Ajouter un ami</v-btn>
            </v-col>
        </v-row>

        <v-dialog v-model="showDeleteModal" max-width="500px">
            <v-card>
                <v-card-title class="headline">Confirmer la suppression</v-card-title>
                <v-card-text>Voulez-vous vraiment supprimer cette amitié ?</v-card-text>
                <v-card-actions>
                    <v-spacer></v-spacer>
                    <v-btn color="blue darken-1" text @click="closeDeleteModal">Annuler</v-btn>
                    <v-btn color="red darken-1" text @click="deleteFriend">Supprimer</v-btn>
                </v-card-actions>
            </v-card>
        </v-dialog>

        <v-dialog v-model="showAddFriendModal" max-width="500px">
            <v-card>
                <v-card-title class="headline">Ajouter un ami</v-card-title>
                <v-card-text>
                    <v-form ref="addFriendForm" v-model="valid">
                        <v-text-field v-model="newFriendEmail" label="Email" :rules="[rules.required, rules.email]"
                            required></v-text-field>
                    </v-form>
                </v-card-text>
                <v-card-actions>
                    <v-spacer></v-spacer>
                    <v-btn color="blue darken-1" text @click="closeAddFriendModal">Annuler</v-btn>
                    <v-btn color="green darken-1" text @click="addFriend">Ajouter</v-btn>
                </v-card-actions>
            </v-card>
        </v-dialog>
    </v-container>
</template>

<script setup>
import { ref, computed, onMounted } from 'vue';
import { useUserStore } from "@/store/userStore";
import authPromise from "@/plugins/keycloak";
import router from "@/router";

const userStore = useUserStore();

const friendRequests = computed(() => {
    return userStore.friendships.filter((f) => !f.isValidated);
});

const friends = computed(() => {
    let friends = [];
    let friendships = userStore.friendships.filter((f) => f.isValidated);
    friendships.forEach((friendship) => {
        if (friendship.friendWhoAsked.id === userStore.currentUser.id) {
            friends.push(friendship.friendWhoGotAsked)
        } else {
            friends.push(friendship.friendWhoAsked)
        }
    });
    return friends;
});

const showDeleteModal = ref(false);
const friendToDelete = ref(null);

const showAddFriendModal = ref(false);
const newFriendEmail = ref('');
const valid = ref(false);
const rules = {
    required: value => !!value || 'Requis.',
    email: value => /.+@.+\..+/.test(value) || 'E-mail invalide.',
};

const getFriendName = (friendship) => {
    const user = userStore.currentUser;
    return friendship.friendWhoGotAsked.id === user.id ? `${friendship.friendWhoGotAsked.firstname} ${friendship.friendWhoGotAsked.lastname}` : `${friendship.friendWhoAsked.firstname} ${friendship.friendWhoAsked.lastname}`;
};

const isRecipient = (friendship) => {
    return friendship.friendWhoGotAsked.id === userStore.currentUser.id;
};

const confirmRequest = async (friendship) => {
    let userId = userStore.currentUser.id;
    let friendId = friendship.friendWhoAsked.id === userId ? friendship.friendWhoGotAsked.id : friendship.friendWhoAsked.id;
    await userStore.acceptFriendship(userId, friendId, userId);
};

const openDeleteModal = (friend) => {
    friendToDelete.value = friend;
    showDeleteModal.value = true;
};

const closeDeleteModal = () => {
    showDeleteModal.value = false;
    friendToDelete.value = null;
};

const deleteFriend = async () => {
    await userStore.removeFriendship(userStore.currentUser.id, friendToDelete.value.id);
    closeDeleteModal();
};

const openAddFriendModal = () => {
    showAddFriendModal.value = true;
};

const closeAddFriendModal = () => {
    showAddFriendModal.value = false;
    newFriendEmail.value = '';
};

const addFriend = async () => {
    if (valid.value) {
        try {
            await userStore.addUserFriendship(userStore.currentUser.id, newFriendEmail.value);
            closeAddFriendModal();
        } catch (error) {
            console.error('Error adding friend:', error);
        }
    }
};

onMounted(async () => {
    authPromise.then(async (auth) => {
        if (auth.isAuthenticated()) {
            const userStore = useUserStore();
            let userIsInDB = await userStore.DoesUserExistInDB(auth.userId());
            if (userIsInDB) {
                let userId = userStore.currentUser?.id;
                if (userId) {
                    await userStore.getFriendships(userId);
                }
            } else {
                router.push({ path: "/subscription_more_infos/my_friends" });
            }
        }
    });
});
</script>

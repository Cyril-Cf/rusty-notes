<template>
    <v-data-table :headers="headers" :items="userStore.friendshipsAwaitingValidation" fixed-footer width="auto"
        no-data-text="Pas de demande en cours" :pageText="'{0}-{1} sur {2}'" items-per-page-text="Item par page"
        height="200px">
        <template v-slot:top>
            <v-toolbar flat>
                <v-toolbar-title>Mes demandes d'amitiée</v-toolbar-title>
                <v-divider class="mx-4" inset vertical></v-divider>
                <v-spacer></v-spacer>
            </v-toolbar>
        </template>
        <template v-slot:item.firstname="{ item }">
            {{ getFirstName(item) }}
        </template>
        <template v-slot:item.lastname="{ item }">
            {{ getLastName(item) }}
        </template>
        <template v-slot:item.email="{ item }">
            {{ getEmail(item) }}
        </template>
        <template v-slot:item.updatedAt="{ item }">
            {{ formatFriendshipDate(item) }}
        </template>
        <template v-slot:item.action="{ item }">
            <v-btn v-if="isRecipient(item)" @click="confirmRequest(item)">Confirmer</v-btn>
            <v-tooltip v-else>
                <template v-slot:activator="{ props }">
                    <span v-bind="props"><v-icon color="primary" size="small">mdi-clock-outline</v-icon></span>
                </template>
                <span>
                    En cours de confirmation
                </span>
            </v-tooltip>
        </template>
    </v-data-table>
</template>

<script lang="ts" setup>
import { formatDate } from "@/service";
import { useUserStore } from "@/store/userStore";
import { Friendship } from '@/types/Friendship';
import { VDataTable } from 'vuetify/labs/VDataTable';

const userStore = useUserStore();
const headers = [
    { title: 'Prénom', key: 'firstname', sortable: false },
    { title: 'Nom', key: 'lastname', sortable: false },
    { title: 'Email', key: 'email', sortable: false },
    { title: 'Date de la demande', key: 'updatedAt', sortable: false },
    { title: 'Action', key: 'action', sortable: false },
];

const formatFriendshipDate = (friendship: Friendship) => {
    if (friendship.updatedAt) {
        return formatDate(friendship.updatedAt as unknown as string);
    }
    return 'Date non trouvée'
}

const getFirstName = (friendship: Friendship) => {
    if (isRecipient(friendship)) {
        return friendship.friendWhoAsked.firstname
    } else {
        return friendship.friendWhoGotAsked.firstname
    }
}

const getLastName = (friendship: Friendship) => {
    if (isRecipient(friendship)) {
        return friendship.friendWhoAsked.lastname
    } else {
        return friendship.friendWhoGotAsked.lastname
    }
}

const getEmail = (friendship: Friendship) => {
    if (isRecipient(friendship)) {
        return friendship.friendWhoAsked.email
    } else {
        return friendship.friendWhoGotAsked.email
    }
}

const confirmRequest = async (friendship: Friendship) => {
    let userId = userStore.currentUser?.id;
    await userStore.acceptFriendship(userId!.toString(), friendship.friendWhoAsked.id, userId!.toString());
};

const isRecipient = (friendship: Friendship) => {
    return friendship.friendWhoGotAsked.id === userStore.currentUser?.id;
};
</script>

<style scoped></style>
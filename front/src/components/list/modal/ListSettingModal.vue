<template>
    <v-card>
        <v-card-text>
            <v-data-table :headers="headers" :items="usersItem" fixed-footer items-per-page-text="Item par page"
                height="400px" :pageText="'{0}-{1} sur {2}'">
                <template v-slot:top>
                    <v-toolbar flat>
                        <v-toolbar-title>Utilisateurs</v-toolbar-title>
                        <v-divider class="mx-4" inset vertical></v-divider>
                        <v-spacer></v-spacer>
                        <InviteFriendToListModal @closeSettingModalEmit="emit('closeSettingsEmit')" />
                    </v-toolbar>
                </template>
                <template v-slot:item.presentation="{ item }">
                    {{ item.fullname }} ({{ item.email }})
                </template>
                <template v-slot:item.action="{ item }">
                    <v-icon v-if="showDeletebutton(item)" color="black" size="small" @click="removeFriend(item)">
                        mdi-delete
                    </v-icon>
                    <v-dialog v-model="openRemoveFriendToListModal" max-width="400px">
                        <RemoveFriendFromListModal
                            @closeRemoveFriendFromListModalEmit="openRemoveFriendToListModal = false"
                            @RemoveFriendConfirmEmit="RemoveFriendConfirm" />
                    </v-dialog>
                </template>
                <template v-slot:item.status="{ item }">
                    <div class="">
                        <v-icon :color="getStatusColor(item)" size="small">
                            <v-tooltip>
                                <template v-slot:activator="{ props }">
                                    <span v-bind="props">
                                        <v-icon :color="getStatusColor(item)" size="small">
                                            {{ getStatusIcon(item) }}
                                        </v-icon>
                                    </span>
                                </template>
                                <span>{{ getTooltipIcon(item) }}</span>
                            </v-tooltip>
                        </v-icon>
                    </div>
                </template>
                <template v-slot:item.permission="{ item }">
                    <v-icon class="mx-1" v-for="permission in getPermissionIcons(item)" :key="permission.text"
                        color="grey" size="small"><v-tooltip>
                            <template v-slot:activator="{ props }">
                                <span v-bind="props"><v-icon color="grey" size="small">
                                        {{ permission.text }}
                                    </v-icon></span>
                            </template>
                            <span>{{ permission.tooltip }}</span>
                        </v-tooltip>
                    </v-icon>
                </template>
            </v-data-table>
        </v-card-text>
        <v-card-actions>
            <v-spacer></v-spacer>
            <v-btn color="error" @click="emit('closeSettingsEmit')">Fermer</v-btn>
        </v-card-actions>
    </v-card>
</template>

<script lang="ts" setup>
import { defineEmits, ref, computed } from 'vue'
import InviteFriendToListModal from './InviteFriendToListModal.vue';
import RemoveFriendFromListModal from './RemoveFriendFromListModal.vue';
import { ListPermission } from '@/types/List';
import { useListStore } from "@/store/listStore";
import { useUserStore } from "@/store/userStore";
import { User } from '@/types/User';
import { VDataTable } from 'vuetify/labs/VDataTable'

const emit = defineEmits(['closeSettingsEmit'])

const listStore = useListStore();
const userStore = useUserStore();
const openRemoveFriendToListModal = ref(false);
const friendToRemove = ref<User | null>(null);

const headers = [
    { title: 'Utilisateur', key: 'presentation', sortable: false },
    { title: 'Droit', key: 'permission', sortable: false },
    { title: 'Statut', key: 'status', sortable: false },
    { title: 'Action', key: 'action', sortable: false },
];

interface userItem {
    id: String;
    firstname: String;
    lastname: String;
    fullname: String;
    email: String;
    keycloakUuid: String;
    validated: boolean;
    permission: ListPermission
}

const showDeletebutton = (user: User) => {
    if (userStore.currentUser) {
        if (userStore.currentUser.id === user.id) {
            return false;
        }
    }
    return true;
}

const RemoveFriendConfirm = async () => {
    if (userStore.currentUser && friendToRemove.value) {
        await listStore.removeAUserFromList(listStore.selectedList!.id, userStore.currentUser?.id, friendToRemove.value.id);
    }
    openRemoveFriendToListModal.value = false;
    emit('closeSettingsEmit');
}

const usersItem = computed(() => {
    let array: userItem[] = [];
    listStore.usersValidated.forEach((user) => {
        array.push({ validated: true, fullname: `${user.firstname} ${user.lastname}`, permission: user.listPermission, ...user });
    })
    listStore.usersAwaitingValidation.forEach((user) => {
        array.push({ validated: false, fullname: `${user.firstname} ${user.lastname}`, permission: user.listPermission, ...user });
    })
    return array;
});

const getStatusIcon = (item: userItem) => {
    if (item.validated) {
        return 'mdi-check-bold';
    } else {
        return 'mdi-account-clock';
    }
}

const getTooltipIcon = (item: userItem) => {
    if (item.validated) {
        return 'Validé';
    } else {
        return 'En attente';
    }
}

const getStatusColor = (item: userItem) => {
    if (item.validated) {
        return 'green';
    } else {
        return 'red';
    }
}

const getPermissionIcons = (item: userItem) => {
    if (item.permission === ListPermission.OWNER) {
        return [
            { text: 'mdi-magnify', tooltip: 'Consulter' },
            { text: 'mdi-pen', tooltip: 'Modifier' },
        ]
    } else if (item.permission === ListPermission.CAN_SEE_AND_MODIFY) {
        return [
            { text: 'mdi-magnify', tooltip: 'Consulter' },
            { text: 'mdi-pen', tooltip: 'Modifier' },
        ]
    } else {
        return [
            { text: 'mdi-magnify', tooltip: 'Consulter' },
        ]
    }
}

const removeFriend = (friend: userItem) => {
    friendToRemove.value = { email: friend.email, firstname: friend.firstname, lastname: friend.lastname, id: friend.id, keycloakUuid: friend.keycloakUuid, listPermission: friend.permission };
    openRemoveFriendToListModal.value = true;
}
</script>

<style scoped></style>
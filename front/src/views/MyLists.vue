<template>
    <div>
        <v-container>
            <v-row>
                <v-col cols="12">
                    <h1>Mes listes</h1>
                </v-col>
                <v-col cols="12" v-for="(list, index) in listStore.ownedLists" :key="index">
                    <v-card>
                        <v-card-title>
                            {{ list.name }} (<v-tooltip>
                                <template v-slot:activator="{ props }">
                                    <span v-bind="props">{{ list.usersValidated.length +
                                        list.usersAwaitingValidation.length }} {{ (list.usersValidated.length +
                                            list.usersAwaitingValidation.length) > 1 ?
                                            "utilisateurs" : "utilisateur" }}</span>
                                </template>
                                <span>
                                    <ul>
                                        <li v-for="user in list.usersValidated" :key="user.id.toString()">{{
                                            user.firstname }} {{
                                                user.lastname }}</li>
                                    </ul>
                                    <ul>
                                        <li v-for="user in list.usersAwaitingValidation" :key="user.id.toString()">{{
                                            user.firstname }} {{
                                                user.lastname }} (en attente de validation)</li>
                                    </ul>
                                </span>
                            </v-tooltip>)
                        </v-card-title>
                        <v-card-text>
                            <v-chip v-for="(tag, index) in list.tags" :key="index" class="ma-1" outlined>{{ tag
                                }}</v-chip>
                        </v-card-text>
                        <v-card-actions>
                            <v-btn @click="goToSingleList(list.id)" color="primary">Détails</v-btn>
                            <v-btn @click="deleteList(list.id)" color="error">Supprimer</v-btn>
                            <v-icon @click="openSettings(list)" class="ml-auto">mdi-cog</v-icon>
                        </v-card-actions>
                    </v-card>
                </v-col>
            </v-row>
            <v-row v-if="listStore.sharedListsValidated.length > 0">
                <v-col cols="12">
                    <h1>Mes listes partagées</h1>
                </v-col>
                <v-col cols="12" v-for="(list, index) in listStore.sharedListsValidated" :key="index">
                    <v-card>
                        <v-card-title>
                            {{ list.name }} (
                            <v-tooltip>
                                <template v-slot:activator="{ props }">
                                    <span v-bind="props">{{ list.usersValidated.length +
                                        list.usersAwaitingValidation.length }} {{ (list.usersValidated.length +
                                            list.usersAwaitingValidation.length) > 1 ?
                                            "utilisateurs" : "utilisateur" }}</span>
                                </template>
                                <span>
                                    <ul>
                                        <li v-for="user in list.usersValidated" :key="user.id.toString()">{{
                                            user.firstname }} {{
                                                user.lastname }}</li>
                                    </ul>
                                    <ul>
                                        <li v-for="user in list.usersAwaitingValidation" :key="user.id.toString()">{{
                                            user.firstname }} {{
                                                user.lastname }} (en attente de validation)</li>
                                    </ul>
                                </span>
                            </v-tooltip>
                            )
                        </v-card-title>
                        <v-card-text>
                            <v-chip v-for="(tag, index) in list.tags" :key="index" class="ma-1" outlined>{{ tag
                                }}</v-chip>
                        </v-card-text>
                        <v-card-actions>
                            <v-btn @click="goToSingleList(list.id)" color="primary">Détails</v-btn>
                        </v-card-actions>
                    </v-card>
                </v-col>
            </v-row>
            <v-row v-if="listStore.sharedListToValidate.length > 0">
                <v-col cols="12">
                    <h1>Mes invitations</h1>
                </v-col>
                <v-col cols="12" v-for="(list, index) in listStore.sharedListToValidate" :key="index">
                    <v-card>
                        <v-card-title>
                            {{ list.name }} (
                            <v-tooltip>
                                <template v-slot:activator="{ props }">
                                    <span v-bind="props">{{ list.usersValidated.length +
                                        list.usersAwaitingValidation.length }} {{ (list.usersValidated.length +
                                            list.usersAwaitingValidation.length) > 1 ?
                                            "utilisateurs" : "utilisateur" }}</span>
                                </template>
                                <span>
                                    <ul>
                                        <li v-for="user in list.usersValidated" :key="user.id.toString()">{{
                                            user.firstname }} {{
                                                user.lastname }}</li>
                                    </ul>
                                    <ul>
                                        <li v-for="user in list.usersAwaitingValidation" :key="user.id.toString()">{{
                                            user.firstname }} {{
                                                user.lastname }} (en attente de validation)</li>
                                    </ul>
                                </span>
                            </v-tooltip>
                            )
                        </v-card-title>
                        <v-card-text>
                            <v-chip v-for="(tag, index) in list.tags" :key="index" class="ma-1" outlined>{{ tag
                                }}</v-chip>
                        </v-card-text>
                        <v-card-actions>
                            <v-btn @click="acceptInvitation(list.id)" color="primary">Accepter de participer</v-btn>
                            <v-btn @click="refuseInvitation(list.id)" color="error">Refuser de participer</v-btn>
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

        <v-dialog v-model="settingsDialog" max-width="600">
            <v-card>
                <v-card-text>
                    <v-data-table :headers="headers" :items="usersItem">
                        <template v-slot:top>
                            <v-toolbar flat>
                                <v-toolbar-title>Utilisateurs</v-toolbar-title>
                                <v-divider class="mx-4" inset vertical></v-divider>
                                <v-spacer></v-spacer>
                                <v-dialog v-model="inviteFriendModal" max-width="500px">
                                    <template v-slot:activator="{ props }">
                                        <v-btn class="mb-2" color="primary" dark v-bind="props">
                                            Inviter un ami
                                        </v-btn>
                                    </template>
                                    <v-card>
                                        <v-card-title>
                                            <span class="text-h5">Inviter un ami à cette liste</span>
                                        </v-card-title>

                                        <v-card-text>
                                            <v-container>
                                                <v-row>
                                                    <v-col cols="12" md="12" sm="12">
                                                        <v-select :items="friendsToInvite"
                                                            :item-title="getFriendFullName" item-value="id"
                                                            no-data-text="Personne à ajouter" v-model="selectedFriend"
                                                            :rules="[rules.required]" required
                                                            label="Sélectionner un ami"></v-select>
                                                    </v-col>
                                                    <v-col cols="12" md="12" sm="12">
                                                        <v-select :items="listPermissionItems"
                                                            v-model="selectedPermission" label="Permissions"
                                                            item-title="text" item-value="value"
                                                            :rules="[rules.required]" return-object single-line
                                                            required></v-select>
                                                    </v-col>
                                                </v-row>
                                            </v-container>
                                        </v-card-text>

                                        <v-card-actions>
                                            <v-spacer></v-spacer>
                                            <v-btn color="blue-darken-1" variant="text" @click="closeInviteFriendModal">
                                                Annuler
                                            </v-btn>
                                            <v-btn color="blue-darken-1" variant="text" @click="saveInviteFriendModal">
                                                Inviter
                                            </v-btn>
                                        </v-card-actions>
                                    </v-card>
                                </v-dialog>
                                <v-dialog v-model="removeFriendModal" max-width="500px">
                                    <v-card>
                                        <v-card-title class="text-h5">Retirer cet ami de la
                                            liste ?</v-card-title>
                                        <v-card-actions>
                                            <v-spacer></v-spacer>
                                            <v-btn color="blue-darken-1" variant="text"
                                                @click="closeRemoveFriendModal">Annuler</v-btn>
                                            <v-btn color="blue-darken-1" variant="text"
                                                @click="RemoveFriendConfirm">OK</v-btn>
                                            <v-spacer></v-spacer>
                                        </v-card-actions>
                                    </v-card>
                                </v-dialog>
                            </v-toolbar>
                        </template>
                        <template v-slot:item.action="{ item }">
                            <v-icon v-if="showDeletebutton(item)" color="black" size="small"
                                @click="removeFriend(item)">
                                mdi-delete
                            </v-icon>
                        </template>
                        <template v-slot:item.status="{ item }">
                            <v-icon :color="getStatusColor(item)" size="small">
                                {{ getStatusIcon(item) }}
                            </v-icon>
                        </template>
                        <template v-slot:item.permission="{ item }">
                            <v-icon v-for="permission in getPermissionIcons(item)" :key="permission.text" color="grey"
                                size="small"><v-tooltip>
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
                    <v-btn color="blue darken-1" @click="closeSettings">Fermer</v-btn>
                </v-card-actions>
            </v-card>
        </v-dialog>
    </div>
</template>

<script lang="ts" setup>
import { ref, onMounted, computed } from 'vue';
import { useListStore } from "@/store/listStore";
import { useUserStore } from "@/store/userStore";
import authPromise from "@/plugins/keycloak";
import router from "@/router";
import { List, ListPermission } from '@/types/List';
import { VDataTable } from 'vuetify/labs/VDataTable'
import { User } from '@/types/User';

const userStore = useUserStore();
const listStore = useListStore();

const settingsDialog = ref(false);
const selectedList = ref<List | undefined>(undefined);
const selectedFriend = ref<String | null>(null);
const friendToRemove = ref<User | null>(null);
const removeFriendModal = ref(false)

const headers = [
    { title: 'Nom', key: 'fullname', sortable: false },
    { title: 'Email', key: 'email', sortable: false },
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

const inviteFriendModal = ref(false);
const closeInviteFriendModal = () => {
    inviteFriendModal.value = false
}
const saveInviteFriendModal = async () => {
    if (selectedList.value && userStore.currentUser && selectedFriend.value && selectedPermission.value) {
        await listStore.inviteUserToMyList(selectedList.value.id, userStore.currentUser?.id.valueOf(), selectedFriend.value, selectedPermission.value.value as ListPermission);
    }
    inviteFriendModal.value = false;
    settingsDialog.value = false;
}

const selectedPermission = ref<ListPermissionInSelect | null>(null);

const rules = {
    required: (value: string) => !!value || 'Ce champ est requis'
};


interface ListPermissionInSelect {
    text: String;
    value: String;
}

const listPermissionItems: ListPermissionInSelect[] = [
    { text: 'Peut voir mais pas modifier', value: ListPermission.CAN_SEE_BUT_NOT_MODIFY },
    { text: 'Peut voir et modifier (mais pas supprimer)', value: ListPermission.CAN_SEE_AND_MODIFY }
];


const removeFriend = (friend: userItem) => {
    friendToRemove.value = { email: friend.email, firstname: friend.firstname, lastname: friend.lastname, id: friend.id, keycloakUuid: friend.keycloakUuid, listPermission: friend.permission };
    removeFriendModal.value = true;
}

const closeRemoveFriendModal = () => {
    removeFriendModal.value = false
}

const friendsToInvite = computed(() => {
    if (selectedList.value) {
        const combinedExcludeList = [...listStore.usersAwaitingValidation, ...listStore.usersValidated];
        return userStore.userFriends.filter(user => !combinedExcludeList.some(excludeUser => excludeUser.id === user.id));
    } else {
        return [];
    }
});

const showDeletebutton = (user: User) => {
    if (userStore.currentUser) {
        if (userStore.currentUser.id === user.id) {
            return false;
        }
    }
    return true;
}

const RemoveFriendConfirm = async () => {
    const listId = selectedList.value?.id;
    if (listId && userStore.currentUser && friendToRemove.value) {
        await listStore.removeAUserFromList(listId, userStore.currentUser?.id, friendToRemove.value.id);
    }
    removeFriendModal.value = false;
    settingsDialog.value = false;
}

const getFriendFullName = (friend: User) => {
    return `${friend.firstname} ${friend.lastname}(${friend.email})`
}

const showAddListDialog = () => {
    router.push({ path: "/add_list" });
};

const deleteList = async (listId: String) => {
    const userId = userStore.currentUser?.id;
    if (userId && listId) {
        await listStore.deleteSelectedList(listId, userId);
    }
};

const goToSingleList = (listId: string) => {
    router.push({ path: `/single_list/${listId}` });
};

const openSettings = async (list: any) => {
    selectedList.value = list;
    await listStore.fetchOne(list.id);
    settingsDialog.value = true;
};

const closeSettings = () => {
    settingsDialog.value = false;
    selectedList.value = undefined;
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
                router.push({ path: "/subscription_more_infos/my_lists" });
            }
        }
    });
});
</script>

<style scoped>
ul {
    list-style-type: none;
    padding-left: 5px;
}
</style>
<template>
    <div>
        <v-dialog v-model="openInviteFriendToListModal" max-width="500px">
            <template v-slot:activator="{ props }">
                <v-btn class="mb-2" color="primary" dark v-bind="props">
                    Inviter un ami
                </v-btn>
            </template>
            <v-card class="px-4 py-4">
                <v-card-title class="d-flex pa-0 pl-4 flex-start">
                    <div>Inviter un ami à cette liste</div>
                </v-card-title>
                <v-card-subtitle class="text-wrap my-2">
                    Choisissez parmi vos amis avec qui vous souhaitez partager cette liste, puis les permissions que
                    cette personne
                    aura (simplement regarder ou bien même la modifier).
                </v-card-subtitle>
                <v-card-text class="pb-0">
                    <v-container>
                        <v-row>
                            <v-col cols="12">
                                <v-select :items="friendsToInvite" :item-title="getFriendFullName" item-value="id"
                                    no-data-text="Personne à ajouter" v-model="selectedFriend" :rules="[rules.required]"
                                    required label="Sélectionner un ami"></v-select>
                            </v-col>
                            <v-col cols="12" class="pb-0">
                                <v-select :items="listPermissionItems" v-model="selectedPermission" label="Permissions"
                                    item-title="text" item-value="value" :rules="[rules.required]" return-object
                                    single-line required></v-select>
                            </v-col>
                        </v-row>
                    </v-container>
                </v-card-text>
                <v-card-actions class="pa-0 d-flex justify-center">
                    <v-btn color="blue-darken-1" variant="text" @click="openInviteFriendToListModal = false">
                        Annuler
                    </v-btn>
                    <v-btn color="blue-darken-1" variant="text" @click="saveInviteFriendModal">
                        Inviter
                    </v-btn>
                </v-card-actions>
            </v-card>
        </v-dialog>
    </div>
</template>

<script lang="ts" setup>
import { computed, defineEmits, ref } from 'vue'
import { useListStore } from "@/store/listStore";
import { useUserStore } from "@/store/userStore";
import { ListPermission } from '@/types/List';
import { User } from '@/types/User';

const emit = defineEmits(['closeSettingModalEmit'])
const listStore = useListStore();
const userStore = useUserStore();
const selectedFriend = ref<String | null>(null);

const saveInviteFriendModal = async () => {
    if (userStore.currentUser && selectedFriend.value && selectedPermission.value) {
        await listStore.inviteUserToMyList(listStore.selectedList!.id, userStore.currentUser?.id.valueOf(), selectedFriend.value, selectedPermission.value.value as ListPermission);
    }
    openInviteFriendToListModal.value = false;
    emit('closeSettingModalEmit');
}

const openInviteFriendToListModal = ref(false);


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


const getFriendFullName = (friend: User) => {
    return `${friend.firstname} ${friend.lastname}(${friend.email})`
}

const friendsToInvite = computed(() => {
    const combinedExcludeList = [...listStore.usersAwaitingValidation, ...listStore.usersValidated];
    return userStore.userFriends.filter(user => !combinedExcludeList.some(excludeUser => excludeUser.id === user.id));
});
</script>

<style scoped></style>
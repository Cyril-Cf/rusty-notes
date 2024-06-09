<template>
    <div>
        <v-card @click="goToSingleList(list.id)" class="pa-2">
            <v-card-title class="pa-0">
                <v-container class="pa-0">
                    <v-row justify="space-between" no-gutters>
                        <v-col cols="12" sm="4">
                            <div>{{ list.name }}</div>
                        </v-col>
                        <v-col @mousedown.stop cols="12" sm="2" class="d-flex flex-end">
                            <v-tooltip>
                                <template v-slot:activator="{ props }">
                                    <span v-bind="props"><v-icon v-if="list.isOwner" @click.stop="deleteList(list.id)"
                                            color="primary" size="small"
                                            class="ml-auto">mdi-delete-circle</v-icon></span>
                                </template>
                                <span>
                                    Supprimer la liste
                                </span>
                            </v-tooltip>

                            <v-tooltip>
                                <template v-slot:activator="{ props }">
                                    <span v-bind="props"><v-icon v-if="list.isOwner" @click.stop="openSettings(list)"
                                            color="primary" size="small">mdi-account-multiple</v-icon></span>
                                </template>
                                <span>
                                    Editer les utilisateurs
                                </span>
                            </v-tooltip>
                        </v-col>
                    </v-row>
                </v-container>
            </v-card-title>
            <v-card-subtitle class="pa-0">
                <v-container class="pa-0">
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
                </v-container>
            </v-card-subtitle>
            <v-card-actions v-if="!list.isValidated">
                <v-btn v-if="!list.isValidated" @click="acceptInvitation(list.id)" color="primary">Accepter de
                    participer</v-btn>
                <v-btn v-if="!list.isValidated" @click="refuseInvitation(list.id)" color="error">Refuser de
                    participer</v-btn>
            </v-card-actions>
        </v-card>
    </div>
</template>

<script lang="ts" setup>
import { defineProps, defineEmits, PropType } from 'vue'
import { List } from '@/types/List';

const emit = defineEmits(['goToSingleListEmit', 'deleteListEmit', 'openSettingsEmit', 'acceptInvitationEmit', 'refuseInvitationEmit'])

const props = defineProps({
    list: {
        type: Object as PropType<List>,
        required: true
    },
});

const deleteList = async (listId: String) => {
    emit('deleteListEmit', listId)
};

const goToSingleList = (listId: string) => {
    if (props.list.isValidated || props.list.isOwner) {
        emit('goToSingleListEmit', listId)
    };
};

const openSettings = async (list: List) => {
    emit('openSettingsEmit', list)
};

const acceptInvitation = async (listId: String) => {
    emit('acceptInvitationEmit', listId)
};

const refuseInvitation = async (listId: String) => {
    emit('refuseInvitationEmit', listId)
};
</script>

<style scoped>
ul {
    list-style-type: none;
    padding-left: 5px;
}
</style>
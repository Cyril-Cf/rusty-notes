<template>
    <div>
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
                <v-btn v-if="list.isOwner" @click="deleteList(list.id)" color="error">Supprimer</v-btn>
                <v-icon v-if="list.isOwner" @click="openSettings(list)" class="ml-auto">mdi-cog</v-icon>
                <v-btn v-if="!list.isValidated" @click="acceptInvitation(list.id)" color="primary">Accepter de
                    participer</v-btn>
                <v-btn v-if="!list.isValidated" @click="refuseInvitation(list.id)" color="error">Refuser de
                    participer</v-btn>
            </v-card-actions>
        </v-card>
    </div>
</template>

<script lang="ts" setup>
import { defineProps, defineEmits, PropType, ref } from 'vue'
import { List } from '@/types/List';

const emit = defineEmits(['goToSingleListEmit', 'deleteListEmit', 'openSettingsEmit', 'acceptInvitationEmit', 'refuseInvitationEmit'])

defineProps({
    list: {
        type: Object as PropType<List>,
        required: true
    },
});

const deleteList = async (listId: String) => {
    emit('deleteListEmit', listId)
};

const goToSingleList = (listId: string) => {
    emit('goToSingleListEmit', listId)
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
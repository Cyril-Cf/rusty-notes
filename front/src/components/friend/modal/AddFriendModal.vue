<template>
    <v-card class="px-4 py-4">
        <v-card-title class="d-flex pa-0 pl-4 flex-start">
            <div>Ajouter un ami</div>
        </v-card-title>
        <v-card-subtitle class="text-wrap my-2">
            Afin d'ajouter un ami, entrez simplement l'email que cette personne a donné au moment de son inscription
            et
            nous la préviendrons pour vous !
        </v-card-subtitle>
        <v-card-text>
            <v-form ref="addFriendForm" v-model="valid">
                <v-text-field v-model="newFriendEmail" label="Son email" :rules="[rules.required, rules.email]"
                    required></v-text-field>
            </v-form>
        </v-card-text>
        <v-card-actions class="pa-0">
            <v-spacer></v-spacer>
            <v-btn color="error" @click="emit('closeAddFriendModalEmit')">Annuler</v-btn>
            <v-btn color="primary" @click="addFriend">Ajouter</v-btn>
            <v-spacer></v-spacer>
        </v-card-actions>
    </v-card>
</template>

<script lang="ts" setup>
import { ref, defineEmits } from 'vue';

const emit = defineEmits(['closeAddFriendModalEmit', 'addFriendEmit'])
const valid = ref(false);
const newFriendEmail = ref('');
const rules = {
    required: (value: any) => !!value || 'Requis.',
    email: (value: string) => /.+@.+\..+/.test(value) || 'E-mail invalide.',
};
const addFriend = () => {
    emit('addFriendEmit', newFriendEmail.value)
}
</script>

<style scoped></style>
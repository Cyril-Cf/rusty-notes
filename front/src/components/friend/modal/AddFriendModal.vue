<template>
    <div>
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
                <v-btn color="blue darken-1" @click="emit('closeAddFriendModalEmit')">Annuler</v-btn>
                <v-btn color="green darken-1" @click="addFriend">Ajouter</v-btn>
            </v-card-actions>
        </v-card>
    </div>
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
<template>
    <div>
        <v-card class="pt-2">
            <v-card-title class="d-flex pa-0 justify-center">
                <div>Retirer cet ami de la
                    liste ?</div>
            </v-card-title>
            <v-card-actions class="pa-0">
                <v-spacer></v-spacer>
                <v-btn color="error" variant="text" @click="emit('closeRemoveFriendFromListModalEmit')">Annuler</v-btn>
                <v-btn color="primary" variant="text" @click="emit('RemoveFriendConfirmEmit')">OK</v-btn>
                <v-spacer></v-spacer>
            </v-card-actions>
        </v-card>
    </div>
</template>

<script lang="ts" setup>
import { defineEmits } from 'vue'
const emit = defineEmits(['closeRemoveFriendFromListModalEmit', 'RemoveFriendConfirmEmit'])
</script>

<style scoped></style>
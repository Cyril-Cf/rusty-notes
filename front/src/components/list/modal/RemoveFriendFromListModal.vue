<template>
    <div>
        <v-card>
            <v-card-title class="text-h5">Retirer cet ami de la
                liste ?</v-card-title>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn color="blue-darken-1" variant="text" @click="closeRemoveFriendModal">Annuler</v-btn>
                <v-btn color="blue-darken-1" variant="text" @click="RemoveFriendConfirm">OK</v-btn>
                <v-spacer></v-spacer>
            </v-card-actions>
        </v-card>
    </div>
</template>

<script lang="ts" setup>
import { defineEmits } from 'vue'

const emit = defineEmits(['closeRemoveFriendFromListModalEmit', 'RemoveFriendConfirmEmit'])

const RemoveFriendConfirm = () => {
    emit('RemoveFriendConfirmEmit')
}

const closeRemoveFriendModal = () => {
    emit('closeRemoveFriendFromListModalEmit')
}
</script>

<style scoped></style>
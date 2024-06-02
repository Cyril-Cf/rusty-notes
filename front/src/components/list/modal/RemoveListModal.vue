<template>
    <div>
        <v-card>
            <v-card-title class="text-h5">Supprimer cette liste ?</v-card-title>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn color="blue-darken-1" variant="text" @click="closeRemoveListModal">Annuler</v-btn>
                <v-btn color="blue-darken-1" variant="text" @click="RemoveListConfirm">OK</v-btn>
                <v-spacer></v-spacer>
            </v-card-actions>
        </v-card>
    </div>
</template>

<script lang="ts" setup>
import { defineEmits } from 'vue'

const emit = defineEmits(['closeRemoveListModalEmit', 'RemoveListConfirmEmit'])

const closeRemoveListModal = () => {
    emit('closeRemoveListModalEmit')
}

const RemoveListConfirm = () => {
    emit('RemoveListConfirmEmit')
}
</script>

<style scoped></style>
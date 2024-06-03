<template>
    <div>
        <v-card>
            <v-card-title class="headline">Confirmer la suppression</v-card-title>
            <v-card-text>Voulez-vous vraiment supprimer cette amitié ?</v-card-text>
            <v-card-actions>
                <v-spacer></v-spacer>
                <v-btn color="blue darken-1" @click="emit('closeDeleteModalEmit')">Annuler</v-btn>
                <v-btn color="red darken-1" @click="emit('deleteFriendEmit')">Supprimer</v-btn>
            </v-card-actions>
        </v-card>
    </div>
</template>

<script lang="ts" setup>
import { defineEmits } from 'vue'
const emit = defineEmits(['closeDeleteModalEmit', 'deleteFriendEmit'])
</script>

<style scoped></style>
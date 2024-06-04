<template>
    <v-card class="px-4 py-4">
        <v-card-title class="d-flex pa-0 pl-4 flex-start">
            <div>Ajouter un élément</div>
        </v-card-title>
        <v-card-subtitle class="text-wrap my-2">
            Ajouter un élément à la liste en donnant son nom, puis son type (case à cocher, bullet-point, etc.)
        </v-card-subtitle>
        <v-card-text>
            <v-text-field v-model="newItemName" :rules="[rules.required]" required
                label="Nom de l'élément"></v-text-field>
            <v-select v-model="newItemType" :rules="[rules.required]" required :items="itemTypeItems" item-title="text"
                item-value="value" label="Type d'élément"></v-select>
        </v-card-text>
        <v-card-actions class="pa-0 d-flex justify-center">
            <v-btn color="error" @click="emit('closeItemModalEmit')">Annuler</v-btn>
            <v-btn color="primary" @click="addItem">Ajouter</v-btn>
        </v-card-actions>
    </v-card>
</template>

<script lang="ts" setup>
import { ItemType, NewItem } from '@/types/Item';
import { ref, defineEmits } from 'vue';
import { useUserStore } from "@/store/userStore";
import { useListStore } from "@/store/listStore";

const emit = defineEmits(['closeItemModalEmit'])

const rules = {
    required: (value: string) => !!value || 'Ce champ est requis'
};
const userStore = useUserStore();
const listStore = useListStore();
const newItemName = ref('');
const newItemType = ref<ItemType | undefined>(undefined);

interface ItemTypeInSelect {
    text: String;
    value: String;
}

const addItem = async () => {
    const userId = userStore.currentUser?.id;
    const listId = listStore.selectedList?.id;
    if (userId && listId && newItemType.value) {
        const newItem: NewItem = { name: newItemName.value.trim(), isChecked: false, listId: listId, itemType: newItemType.value };
        await listStore.addItemToList(listId, newItem);
        newItemName.value = '';
        emit('closeItemModalEmit');
    }
}

const itemTypeItems: ItemTypeInSelect[] = [
    { text: 'Case à cocher', value: ItemType.CHECKBOX },
    { text: 'Puce', value: ItemType.BULLET_POINT }
];
</script>

<style scoped></style>
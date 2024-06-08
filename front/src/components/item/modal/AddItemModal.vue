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
            <v-select v-model="newItemTypeId" :rules="[rules.required]" required :items="itemTypeItems"
                item-title="text" item-value="value" label="Type d'élément"></v-select>
        </v-card-text>
        <v-card-actions class="pa-0 d-flex justify-center">
            <v-btn color="error" @click="emit('closeItemModalEmit')">Annuler</v-btn>
            <v-btn color="primary" @click="addItem">Ajouter</v-btn>
        </v-card-actions>
    </v-card>
</template>

<script lang="ts" setup>
import { NewItem } from '@/types/Item';
import { ref, defineEmits, computed } from 'vue';
import { useUserStore } from "@/store/userStore";
import { useListStore } from "@/store/listStore";

const emit = defineEmits(['closeItemModalEmit'])

const rules = {
    required: (value: string) => !!value || 'Ce champ est requis'
};
const userStore = useUserStore();
const listStore = useListStore();
const newItemName = ref('');
const newItemTypeId = ref<String | undefined>(undefined);

interface ItemTypeInSelect {
    text: String;
    value: String;
}

const addItem = async () => {
    const userId = userStore.currentUser?.id;
    const listId = listStore.selectedList?.id;
    if (userId && listId && newItemTypeId.value) {
        const newItem: NewItem = { content: "test", listId: listId, itemTypeId: newItemTypeId.value.valueOf() };
        await listStore.addItemToList(listId, newItem);
        newItemName.value = '';
        emit('closeItemModalEmit');
    }
}

const itemTypeItems = computed<ItemTypeInSelect[]>(() => {
    if (listStore.selectedList && listStore.selectedList.listType.allowedItemTypes.length > 0) {
        return listStore.selectedList!.listType.allowedItemTypes.map(itemType => ({
            text: itemType.itemTypeVariation,
            value: itemType.id
        }));
    } else {
        return [];
    }
});

console.log(itemTypeItems.value)

</script>

<style scoped></style>
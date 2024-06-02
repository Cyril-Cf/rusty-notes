<template>
    <v-container>
        <v-card v-if="listStore.selectedList">
            <v-card-title>{{ listStore.selectedList.name }} - {{ permissionText }}</v-card-title>
            <v-card-text>
                <template v-for="(item, index) in listStore.selectedItems" :key="index">
                    <v-row class="d-flex align-center ma-1">
                        <v-col>
                            <template v-if="item.itemType === ItemType.CHECKBOX">
                                <v-checkbox v-model="item.isChecked" :readonly="!canModify()"
                                    @click="updateItem(index)"></v-checkbox>
                            </template>
                            <template v-else-if="item.itemType === ItemType.BULLET_POINT">
                                <v-icon>mdi-circle-small</v-icon> {{ item.name }}
                            </template>
                        </v-col>
                        <v-col v-if="item.itemType === ItemType.CHECKBOX">
                            {{ item.name }}
                        </v-col>
                        <v-col v-if="canModify()">
                            <v-icon @click="deleteItem(index)" class="ml-1">mdi-close</v-icon>
                        </v-col>
                    </v-row>
                </template>
            </v-card-text>
            <v-card-actions v-if="canModify()">
                <v-btn color="primary" @click="addItemDialog = true">Ajouter un élément</v-btn>
                <v-btn color="error" @click="deleteList">Supprimer la liste</v-btn>
            </v-card-actions>
        </v-card>

        <v-dialog v-model="addItemDialog" max-width="500px">
            <v-card>
                <v-card-title>Ajouter un élément</v-card-title>
                <v-card-text>
                    <v-text-field v-model="newItemName" label="Nom de l'élément"></v-text-field>
                    <v-select v-model="newItemType" :rules="[rules.required]" :items="itemTypeItems" item-title="text"
                        item-value="value" label="Type d'élément" required></v-select>
                </v-card-text>
                <v-card-actions>
                    <v-btn color="primary" @click="addItem">Ajouter</v-btn>
                    <v-btn color="error" @click="addItemDialog = false">Annuler</v-btn>
                </v-card-actions>
            </v-card>
        </v-dialog>
    </v-container>
</template>

<script lang="ts" setup>
import { useListStore } from "@/store/listStore";
import { useUserStore } from "@/store/userStore";
import { computed, onMounted, ref } from "vue";
import authPromise from "@/plugins/keycloak";
import router from "@/router";
import { useRoute } from 'vue-router';
import { NewItem, ItemType, Item } from '@/types/Item';
import { ListPermission } from "@/types/List";

const userStore = useUserStore();
const listStore = useListStore();
const addItemDialog = ref(false);
const newItemName = ref('');
const newItemType = ref<ItemType | undefined>(undefined);
const route = useRoute();

const rules = {
    required: (value: string) => !!value || 'Ce champ est requis'
};

interface ItemTypeInSelect {
    text: String;
    value: String;
}

const canModify = () => {
    if (listStore.selectedList?.isOwner || listStore.selectedList?.listPermission === ListPermission.CAN_SEE_AND_MODIFY) {
        return true;
    }
    return false
}

const permissionText = computed(() => {
    if (listStore.selectedList?.isOwner) {
        return 'Propriétaire';
    }
    if (listStore.selectedList?.listPermission === ListPermission.CAN_SEE_AND_MODIFY) {
        return 'Invité (peut modifier)';
    }
    if (listStore.selectedList?.listPermission === ListPermission.CAN_SEE_BUT_NOT_MODIFY) {
        return 'Invité (ne peut pas modifier)';
    }
    return ''
});

const deleteList = async () => {
    const userId = userStore.currentUser?.id;
    const listId = listStore.selectedList?.id;
    if (userId && listId) {
        await listStore.deleteSelectedList(listId, userId);
        router.push({ path: "/my_lists" });
    }
}

const updateItem = async (index: number) => {
    const listId = listStore.selectedList?.id;
    const item = listStore.selectedItems?.at(index);
    if (listId && item) {
        const itemToUpdate: Item = { id: item.id, name: item.name, isChecked: !item.isChecked.valueOf(), listId: item.listId, itemType: item.itemType };
        await listStore.updateItemFromList(itemToUpdate, listId);
    }
}

const deleteItem = async (index: number) => {
    const listId = listStore.selectedList?.id;
    const itemId = listStore.selectedList?.items.at(index)?.id;
    if (listId && itemId) {
        await listStore.deleteItemFromList(itemId, listId);
    }
}

const addItem = async () => {
    const userId = userStore.currentUser?.id;
    const listId = listStore.selectedList?.id;
    if (userId && listId && newItemType.value) {
        const newItem: NewItem = { name: newItemName.value.trim(), isChecked: false, listId: listId, itemType: newItemType.value };
        await listStore.addItemToList(listId, newItem);
        newItemName.value = '';
        addItemDialog.value = false;
    }
}

const itemTypeItems: ItemTypeInSelect[] = [
    { text: 'Case à cocher', value: ItemType.CHECKBOX },
    { text: 'Puce', value: ItemType.BULLET_POINT }
];

onMounted(async () => {
    authPromise.then(async (auth) => {
        if (auth.isAuthenticated()) {
            const userStore = useUserStore();
            let userIsInDB = await userStore.DoesUserExistInDB(auth.userId()!);
            if (userIsInDB) {
                const listStore = useListStore();
                const listId = route.params.listId as String;
                await listStore.fetchOne(listId);
            } else {
                router.push({ path: "/subscription_more_infos/my_lists" });
            }
        }
    });
});
</script>

<style scoped></style>
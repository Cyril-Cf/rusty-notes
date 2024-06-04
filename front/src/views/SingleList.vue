<template>
    <v-container class="my-5 mx-auto" style="width: 80vw;" fluid>
        <v-row no-gutters class="mb-5">
            <ButtonGoBackToMyLists />
        </v-row>
        <v-row>
            <v-col v-if="listStore.selectedList" cols="12">
                <v-data-table :headers="headers" :items="listStore.selectedList.items" fixed-footer width="auto"
                    no-data-text="Pas d'élément dans la liste" :pageText="'{0}-{1} sur {2}'"
                    items-per-page-text="Item par page" height="450px">
                    <template v-slot:top>
                        <v-toolbar flat>
                            <v-toolbar-title><span clas="text-h1">{{ listStore.selectedList.name }}</span> <span
                                    class="text-subtitle-1">{{
                                        permissionText }}</span></v-toolbar-title>
                            <v-divider class="mx-4" inset vertical></v-divider>
                            <v-spacer></v-spacer>
                            <v-tooltip>
                                <template v-slot:activator="{ props }">
                                    <span v-bind="props"><v-icon @click="addItemDialog = true" color="primary"
                                            class="pr-8">
                                            mdi-plus-box
                                        </v-icon></span>
                                </template>
                                <span>
                                    Ajouter un élément à la liste
                                </span>
                            </v-tooltip>

                        </v-toolbar>
                    </template>
                    <template v-slot:item.box="{ item }">
                        <div v-if="item.itemType === ItemType.CHECKBOX">
                            <v-checkbox hide-details v-model="item.isChecked" :readonly="!canModify()"
                                @click="updateItem(item)"></v-checkbox>
                        </div>
                        <div v-else-if="item.itemType === ItemType.BULLET_POINT">
                            <v-icon>mdi-circle-small</v-icon> {{ item.name }}
                        </div>
                    </template>
                    <template v-slot:item.action="{ item }">
                        <v-tooltip v-if="canModify()">
                            <template v-slot:activator="{ props }">
                                <span v-bind="props"><v-icon @click="deleteItem(item)"
                                        class="ml-1">mdi-close</v-icon></span>
                            </template>
                            <span>
                                Supprimer cet élément
                            </span>
                        </v-tooltip>
                    </template>
                </v-data-table>
            </v-col>
        </v-row>
        <v-dialog v-model="addItemDialog" max-width="500px">
            <AddItemModal @closeItemModalEmit="addItemDialog = false" />
        </v-dialog>
        <v-dialog v-model="removeListDialog" max-width="300px">
            <RemoveListModal @closeRemoveListModalEmit="removeListDialog = false" @RemoveListConfirmEmit="deleteList" />
        </v-dialog>
    </v-container>
</template>

<script lang="ts" setup>
import AddItemModal from '@/components/item/modal/AddItemModal.vue'
import { useListStore } from "@/store/listStore";
import { useUserStore } from "@/store/userStore";
import { computed, onMounted, ref } from "vue";
import authPromise from "@/plugins/keycloak";
import router from "@/router";
import { useRoute } from 'vue-router';
import { ItemType, Item } from '@/types/Item';
import { ListPermission } from "@/types/List";
import ButtonGoBackToMyLists from "@/components/list/common/ButtonGoBackToMyLists.vue";
import { VDataTable } from 'vuetify/labs/VDataTable';
import RemoveListModal from '@/components/list/modal/RemoveListModal.vue';

const userStore = useUserStore();
const listStore = useListStore();
const addItemDialog = ref(false);
const removeListDialog = ref(false);
const route = useRoute();

const headers = [
    { title: 'Element', key: 'box', sortable: false },
    { title: 'Nom', key: 'name', sortable: false },
    { title: 'Action', key: 'action', sortable: false },
];

const canModify = () => {
    if (listStore.selectedList?.isOwner || listStore.selectedList?.listPermission === ListPermission.CAN_SEE_AND_MODIFY) {
        return true;
    }
    return false
}

const permissionText = computed(() => {
    if (listStore.selectedList?.isOwner) {
        return '(propriétaire de la liste)';
    }
    if (listStore.selectedList?.listPermission === ListPermission.CAN_SEE_AND_MODIFY) {
        return '(invité - peut modifier)';
    }
    if (listStore.selectedList?.listPermission === ListPermission.CAN_SEE_BUT_NOT_MODIFY) {
        return '(invité - ne peut pas modifier)';
    }
    return ''
});

const deleteList = async () => {
    const userId = userStore.currentUser?.id;
    const listId = listStore.selectedList?.id;
    if (userId && listId) {
        await listStore.deleteSelectedList(listId, userId);
        router.push({ path: "/my_notes" });
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
                router.push({ path: "/subscription_more_infos/my_notes" });
            }
        }
    });
});
</script>

<style scoped></style>
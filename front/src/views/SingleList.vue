<template>
    <v-container class="my-5 mx-auto" style="width: 80vw;" fluid>
        <v-row justify="space-between" no-gutters class="mb-5">
            <v-col cols="4">
                <ButtonGoBackToMyLists />
            </v-col>
            <v-col cols="1" class="d-flex justify-center align-center">
                <v-icon color="grey" @click="settingsDialog = true" size="small"><v-tooltip>
                        <template v-slot:activator="{ props }">
                            <span v-bind="props"><v-icon color="grey" size="small">
                                    mdi-cog
                                </v-icon></span>
                        </template>
                        <span>Paramètres</span>
                    </v-tooltip>
                </v-icon>
            </v-col>
            <v-dialog v-model="settingsDialog" max-width="600px">
                <ListSettingModal @closeSettingsEmit="refreshList" />
            </v-dialog>
        </v-row>
        <v-row>
            <v-col v-if="listStore.selectedList" cols="12">
                <v-data-table :headers="headers" :items="listStore.selectedList.items" fixed-footer width="auto"
                    no-data-text="Pas d'élément dans la liste" :pageText="'{0}-{1} sur {2}'"
                    items-per-page-text="Item par page" height="450px">
                    <template v-slot:top>
                        <v-toolbar flat>
                            <v-toolbar-title><span clas="text-h1">{{ listStore.selectedList.name }}</span> <span
                                    :class="mdAndUp ? 'text-subtitle-1' : 'text-subtitle-2'">{{
                                        permissionText }}</span></v-toolbar-title>
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
                    <template v-slot:item.content="{ item }">
                        <div v-if="item.content">
                            {{ item.content }}
                        </div>
                    </template>
                    <template v-slot:item.box="{ item }">
                        <div v-if="item.itemType.itemTypeVariation === ItemTypeVariation.CHECKBOX">
                            <v-checkbox hide-details v-model="item.isChecked" :readonly="!canModify()"></v-checkbox>
                        </div>
                        <div v-else-if="item.itemType.itemTypeVariation === ItemTypeVariation.DEADLINE_DATE">
                            {{ convertDateFormatToDisplay(item.deadlineDate) }}
                        </div>

                        <div v-else-if="item.itemType.itemTypeVariation === ItemTypeVariation.MEDIA_URL">
                            {{ item.mediaUrl }}
                        </div>

                        <div v-else-if="item.itemType.itemTypeVariation === ItemTypeVariation.PERSON_IN_CHARGE">
                            {{ item.personInCharge }}
                        </div>

                        <div v-else-if="item.itemType.itemTypeVariation === ItemTypeVariation.PRIORITY">
                            {{ getPriorityTypeText(item.priorityType) }}
                        </div>

                        <div v-else-if="item.itemType.itemTypeVariation === ItemTypeVariation.WEBSITE_URL">
                            {{ item.websiteUrl }}
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
import ListSettingModal from '@/components/list/modal/ListSettingModal.vue';
import { useListStore } from "@/store/listStore";
import { useUserStore } from "@/store/userStore";
import { computed, onMounted, ref } from "vue";
import { Item, ItemTypeVariation, PriorityType } from "@/types/Item";
import authPromise from "@/plugins/keycloak";
import router from "@/router";
import { useRoute } from 'vue-router';
import { ListPermission } from "@/types/List";
import ButtonGoBackToMyLists from "@/components/list/common/ButtonGoBackToMyLists.vue";
import { VDataTable } from 'vuetify/labs/VDataTable';
import RemoveListModal from '@/components/list/modal/RemoveListModal.vue';
import { useDisplay } from 'vuetify'
const { mdAndUp } = useDisplay()
const userStore = useUserStore();
const listStore = useListStore();
const addItemDialog = ref(false);
const removeListDialog = ref(false);
const route = useRoute();
const settingsDialog = ref(false);

const headers = [
    { title: 'Nom', key: 'content', sortable: false },
    { title: 'Element', key: 'box', sortable: false },
    { title: 'Action', key: 'action', sortable: false },
];

const getPriorityTypeText = (priorityType: PriorityType) => {
    switch (priorityType) {
        case PriorityType.LOW: return "Priorité faible";
        case PriorityType.MIDDLE: return "Priorité moyenne";
        case PriorityType.HIGH: return "Priorité haute";
        default: return "";
    }
}

const canModify = () => {
    if (listStore.selectedList?.isOwner || listStore.selectedList?.listPermission === ListPermission.CAN_SEE_AND_MODIFY) {
        return true;
    }
    return false
}

const refreshList = async () => {
    await listStore.fetchOne(listStore.selectedList!.id);
    settingsDialog.value = false;
}

const convertDateFormatToDisplay = (dateString: string) => {
    const [year, month, day] = dateString.split('-');
    return `${day}/${month}/${year}`;
}

const permissionText = computed(() => {
    if (listStore.selectedList?.isOwner) {
        return '(Propriétaire)';
    }
    if (listStore.selectedList?.listPermission === ListPermission.CAN_SEE_AND_MODIFY) {
        return '(Peut modifier)';
    }
    if (listStore.selectedList?.listPermission === ListPermission.CAN_SEE_BUT_NOT_MODIFY) {
        return '(Ne peut pas modifier)';
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

// const updateItem = async (index: number) => {
//     const listId = listStore.selectedList?.id;
//     const item = listStore.selectedItems?.at(index);
//     if (listId && item) {
//         const itemToUpdate: Item = { id: item.id, name: item.name, isChecked: !item.isChecked.valueOf(), listId: item.listId, itemType: item.itemType };
//         await listStore.updateItemFromList(itemToUpdate, listId);
//     }
// }

const deleteItem = async (item: Item) => {
    const listId = listStore.selectedList?.id;
    if (listId) {
        await listStore.deleteItemFromList(item.id, listId);
    }
}

onMounted(async () => {
    authPromise.then(async (auth) => {
        if (auth.isAuthenticated()) {
            const listStore = useListStore();
            const listId = route.params.listId as String;
            await listStore.fetchOne(listId);
        }
    });
});
</script>

<style scoped></style>
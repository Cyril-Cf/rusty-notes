<template>
    <v-card class="px-4 py-4">
        <v-card-title class="d-flex pa-0 pl-4 flex-start">
            <div>Ajouter un élément</div>
        </v-card-title>
        <v-card-subtitle class="text-wrap my-2">
            Ajouter un élément à la liste en donnant d'abord son type (case à cocher, bullet-point, etc.) et des infos
            complémentaires !
        </v-card-subtitle>
        <v-card-text>
            <v-select v-model="newItemTypeId" :rules="[rules.required]" required :items="itemTypeItems"
                item-title="text" item-value="value" label="Type d'élément"></v-select>
            <v-text-field v-if="newItemTypeId && selectedItemType === ItemTypeVariation.CONTENT
                || newItemTypeId && selectedItemType === ItemTypeVariation.CHECKBOX
                || newItemTypeId && selectedItemType === ItemTypeVariation.DEADLINE_DATE
                || newItemTypeId && selectedItemType === ItemTypeVariation.MEDIA_URL
                || newItemTypeId && selectedItemType === ItemTypeVariation.WEBSITE_URL
                || newItemTypeId && selectedItemType === ItemTypeVariation.PRIORITY
                || newItemTypeId && selectedItemType === ItemTypeVariation.PERSON_IN_CHARGE" v-model="newItemContent"
                :rules="[rules.required]" required label="Texte affiché"></v-text-field>

            <v-select v-if="newItemTypeId && selectedItemType === ItemTypeVariation.PRIORITY"
                v-model="newItemPriorityType" :rules="[rules.required]" required :items="priorityTypeItems"
                item-title="text" item-value="value" label="Priorité"></v-select>

            <v-text-field v-if="newItemTypeId && (selectedItemType === ItemTypeVariation.DEADLINE_DATE)"
                v-model="newItemDeadlineDate" :rules="[rules.required, rules.dateFormat]" required
                label="Date au format DD/MM/YYYY"></v-text-field>

            <v-text-field v-if="newItemTypeId && selectedItemType === ItemTypeVariation.MEDIA_URL"
                v-model="newItemMediaUrl" :rules="[rules.required]" required label="Url de l'image"></v-text-field>

            <v-text-field v-if="newItemTypeId && selectedItemType === ItemTypeVariation.WEBSITE_URL"
                v-model="newItemWebsiteUrl" :rules="[rules.required]" required label="Url du site web"></v-text-field>

            <v-text-field v-if="newItemTypeId && selectedItemType === ItemTypeVariation.PERSON_IN_CHARGE"
                v-model="newItemPersonInCharge" :rules="[rules.required]" required
                label="Personne(s) responsable(s)"></v-text-field>
        </v-card-text>
        <v-card-actions class="pa-0 d-flex justify-center">
            <v-btn color="error" @click="emit('closeItemModalEmit')">Annuler</v-btn>
            <v-btn color="primary" @click="addItem">Ajouter</v-btn>
        </v-card-actions>
    </v-card>
</template>

<script lang="ts" setup>
import { ItemTypeVariation, NewItem, PriorityType } from '@/types/Item';
import { ref, defineEmits, computed } from 'vue';
import { useUserStore } from "@/store/userStore";
import { useListStore } from "@/store/listStore";

const emit = defineEmits(['closeItemModalEmit']);

const rules = {
    required: (value: string) => !!value || 'Ce champ est requis',
    dateFormat: (value: string) => {
        const regex = /^(0[1-9]|[12][0-9]|3[01])\/(0[1-9]|1[0-2])\/(19|20)\d\d$/;
        return regex.test(value) || 'La date doit être au format DD/MM/YYYY';
    },

};
const userStore = useUserStore();
const listStore = useListStore();
const newItemTypeId = ref<string | undefined>(undefined);

const selectedItemType = computed<ItemTypeVariation | undefined>(() => {
    if (newItemTypeId.value) {
        const itemType = listStore.selectedList!.listType.allowedItemTypes.find(itemtype => itemtype.id === newItemTypeId.value);
        return itemType ? itemType.itemTypeVariation : undefined;
    }
    return undefined;
});

const newItemContent = ref('');
const newItemDeadlineDate = ref('');
const newItemMediaUrl = ref('');
const newItemWebsiteUrl = ref('');
const newItemPersonInCharge = ref('');
const newItemPriorityType = ref('');

interface ItemTypeInSelect {
    text: string;
    value: string;
}

const priorityTypeItems: ItemTypeInSelect[] = [
    { text: 'Priorité faible', value: PriorityType.LOW },
    { text: 'Priorité moyenne', value: PriorityType.MIDDLE },
    { text: 'Priorité haute', value: PriorityType.HIGH },
];

const convertDateFormatToInsertDB = (dateString: string) => {
    const [day, month, year] = dateString.split('/');
    return `${year}-${month}-${day}`;
}

const addItem = async () => {
    const listId = listStore.selectedList!.id;
    if (selectedItemType.value) {
        const newItem: NewItem = {
            content: newItemContent.value != '' ? newItemContent.value : undefined,
            deadlineDate: newItemDeadlineDate.value != '' ? convertDateFormatToInsertDB(newItemDeadlineDate.value) : undefined,
            isChecked: false,
            mediaUrl: newItemMediaUrl.value != '' ? newItemMediaUrl.value : undefined,
            personInCharge: newItemPersonInCharge.value != '' ? newItemPersonInCharge.value : undefined,
            priorityType: newItemPriorityType.value != '' ? newItemPriorityType.value as PriorityType : undefined,
            websiteUrl: newItemWebsiteUrl.value != '' ? newItemWebsiteUrl.value : undefined,
            itemTypeId: newItemTypeId.value!,
            listId
        };
        await listStore.addItemToList(listId, newItem);
        newItemContent.value = '';
        emit('closeItemModalEmit');
    }
};

const getItemTypeVariationText = (itemTypeVariation: ItemTypeVariation) => {
    switch (itemTypeVariation) {
        case ItemTypeVariation.CHECKBOX: return "Case à cocher";
        case ItemTypeVariation.CONTENT: return "Contenu texte";
        case ItemTypeVariation.DEADLINE_DATE: return "Date limite";
        case ItemTypeVariation.MEDIA_URL: return "Url du média image";
        case ItemTypeVariation.PERSON_IN_CHARGE: return "Responsable de la tâche";
        case ItemTypeVariation.WEBSITE_URL: return "Url du site web";
        case ItemTypeVariation.PRIORITY: return "Tâche priorisable";
        default: return "";
    }
};

const itemTypeItems = computed<ItemTypeInSelect[]>(() => {
    if (listStore.selectedList && listStore.selectedList.listType.allowedItemTypes.length > 0) {
        return listStore.selectedList.listType.allowedItemTypes.map(itemType => ({
            text: getItemTypeVariationText(itemType.itemTypeVariation),
            value: itemType.id
        }));
    } else {
        return [];
    }
});
</script>

<style scoped></style>

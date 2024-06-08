<template>
    <div>
        <v-form ref="form" v-model="valid">
            <v-text-field v-model="newList.name" label="Nom de la liste" :rules="[rules.required]"
                required></v-text-field>

            <v-select v-model="newList.listType" :items="listTypeItems" label="Type de liste" item-title="text"
                item-value="value" :rules="[rules.required]" return-object single-line required></v-select>

            <v-btn @click="submit" :disabled="!valid" color="primary">Créer ma Liste</v-btn>
        </v-form>
    </div>
</template>

<script lang="ts" setup>
import { ref } from 'vue';
import { useListStore } from "@/store/listStore";
import { useUserStore } from "@/store/userStore";
import { ListType, NewList } from '@/types/List';
import router from "@/router";

const listStore = useListStore();
const userStore = useUserStore();

const newList = ref({
    name: '',
    listType: undefined as ListTypeInSelect | undefined
});

const valid = ref<boolean>(false);

const rules = {
    required: (value: string) => !!value || 'Ce champ est requis'
};

interface ListTypeInSelect {
    text: String;
    value: String;
}

const listTypeItems: ListTypeInSelect[] = [
    { text: 'À faire', value: ListType.TO_DO },
    { text: 'À acheter', value: ListType.TO_BUY }
];

const submit = async () => {
    if (valid.value) {
        let userId = userStore.currentUser?.id;
        if (userId) {

            const input: NewList = {
                listType: newList.value.listType?.value as ListType,
                name: newList.value.name,
                userId: userId.toString()
            };
            await listStore.createNewList(input);
            router.push({ path: '/my_notes' });
        }

    }
};
</script>

<style scoped></style>
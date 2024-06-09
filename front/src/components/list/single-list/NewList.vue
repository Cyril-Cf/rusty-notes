<template>
    <div>
        <v-form ref="form" v-model="valid" class="mt-6">
            <div>
                <div class="text-h6">
                    Type de la liste de notes
                </div>
                <div class="text-subtitle-4 my-4">Choisissez un type de liste en premier</div>
                <v-select v-model="newList.listType" :items="listTypeItems" label="Type de liste" item-title="text"
                    item-value="value" :rules="[rules.required]" return-object single-line required></v-select>
            </div>


            <div v-if="newList.listType">
                <div class="text-h6">
                    Nom de la liste de notes
                </div>
                <div class="text-subtitle-4 my-4">Quel sera le nom de la liste ?</div>
                <v-text-field v-model="newList.name" label="Nom de la liste" :rules="[rules.required]"
                    required></v-text-field>
            </div>
            <v-btn @click="submit" :disabled="!valid" color="primary">Créer ma Liste</v-btn>
        </v-form>
    </div>
</template>

<script lang="ts" setup>
import { computed, ref } from 'vue';
import { useListStore } from "@/store/listStore";
import { useUserStore } from "@/store/userStore";
import { NewList } from '@/types/List';
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

const listTypeItems = computed<ListTypeInSelect[]>(() => {
    if (listStore.listTypes.length > 0) {
        return listStore.listTypes.map(listType => ({
            text: listType.name,
            value: listType.id
        }));
    } else {
        return [];
    }
});

const submit = async () => {
    if (valid.value) {
        let userId = userStore.currentUser?.id;
        if (userId) {

            const input: NewList = {
                listTypeId: newList.value.listType!.value.valueOf(),
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
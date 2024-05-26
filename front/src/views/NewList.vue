<template>
    <v-container>
        <h1>Créer une nouvelle liste</h1>
        <v-form ref="form" v-model="valid">
            <v-text-field v-model="newList.name" label="Nom de la liste" :rules="[rules.required]"
                required></v-text-field>

            <v-select v-model="newList.listType" :items="listTypeItems" label="Type de liste" item-title="text"
                item-value="value" :rules="[rules.required]" return-object single-line required></v-select>

            <v-btn @click="submit" :disabled="!valid" color="primary">Créer ma Liste</v-btn>
        </v-form>
    </v-container>
</template>

<script lang="ts" setup>
import { ref, onMounted } from 'vue';
import { useListStore } from "@/store/listStore";
import { useUserStore } from "@/store/userStore";
import { List, ListType, NewList } from '@/types/List';
import authPromise from "@/plugins/keycloak";
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
            router.push({ path: '/my_lists' });
        }

    }
};
onMounted(async () => {
    authPromise.then(async (auth) => {
        if (auth.isAuthenticated()) {
            const userStore = useUserStore();
            let userIsInDB = await userStore.DoesUserExistInDB(auth.userId()!);
            if (!userIsInDB) {
                router.push({ path: "/subscription_more_infos/add_list" });
            }
        }
    });
});
</script>

<style scoped></style>
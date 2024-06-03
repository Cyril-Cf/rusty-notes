<template>
    <div>
        <v-list-item>
            <v-list-item-title>{{ friend.firstname }} {{ friend.lastname }}</v-list-item-title>
            <v-list-item-action>
                <v-btn color="red" @click="showDeleteModal = true">Supprimer</v-btn>
                <v-dialog v-model="showDeleteModal" max-width="500px">
                    <DeleteFriendModal @closeDeleteModalEmit="showDeleteModal = false"
                        @deleteFriendEmit="deleteFriend" />
                </v-dialog>
            </v-list-item-action>
        </v-list-item>

    </div>
</template>

<script lang="ts" setup>
import DeleteFriendModal from '../modal/DeleteFriendModal.vue';
import { defineProps, defineEmits, PropType, ref } from 'vue'
import { useUserStore } from "@/store/userStore";
import { User } from '@/types/User';

const userStore = useUserStore();
const emit = defineEmits(['exampleEmit'])
const showDeleteModal = ref(false);

const props = defineProps({
    friend: {
        type: Object as PropType<User>,
        required: true
    },
});

const deleteFriend = async () => {
    await userStore.removeFriendship(userStore.currentUser!.id.toString(), props.friend.id);
    showDeleteModal.value = false;
};
</script>

<style scoped></style>
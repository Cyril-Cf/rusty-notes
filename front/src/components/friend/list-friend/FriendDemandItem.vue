<template>
    <div>
        <v-list-item>
            <v-list-item-title>{{ getFriendName() }}</v-list-item-title>
            <v-list-item-action v-if="isRecipient()">
                <v-btn @click="confirmRequest()">Confirmer</v-btn>
            </v-list-item-action>
            <v-list-item-subtitle v-else>En attente de confirmation</v-list-item-subtitle>
        </v-list-item>
    </div>
</template>

<script lang="ts" setup>
import { defineProps, defineEmits, PropType } from 'vue'
import { Friendship } from '@/types/Friendship';
import { useUserStore } from "@/store/userStore";

const userStore = useUserStore();
const emit = defineEmits(['exampleEmit'])

const props = defineProps({
    friendship: {
        type: Object as PropType<Friendship>,
        required: true
    },
});


const getFriendName = () => {
    return props.friendship.friendWhoGotAsked.id === userStore.currentUser?.id ? `${props.friendship.friendWhoGotAsked.firstname} ${props.friendship.friendWhoGotAsked.lastname}` : `${props.friendship.friendWhoAsked.firstname} ${props.friendship.friendWhoAsked.lastname}`;
};

const isRecipient = () => {
    return props.friendship.friendWhoGotAsked.id === userStore.currentUser?.id;
};

const confirmRequest = async () => {
    let userId = userStore.currentUser?.id;
    let friendId = props.friendship.friendWhoAsked.id === userId ? props.friendship.friendWhoGotAsked.id : props.friendship.friendWhoAsked.id;
    await userStore.acceptFriendship(userId!.toString(), friendId, userId!.toString());
};
</script>

<style scoped></style>
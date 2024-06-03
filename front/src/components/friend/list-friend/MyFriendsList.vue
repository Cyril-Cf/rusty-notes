<template>
    <div>
        <v-list>
            <FriendItem v-for="friend in friends" :key="friend.id.toString()" :friend="friend" />
        </v-list>
    </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import { useUserStore } from "@/store/userStore";
import { User } from '@/types/User';
import FriendItem from './FriendItem.vue';

const userStore = useUserStore();

const friends = computed(() => {
    let friends: User[] = [];
    let friendships = userStore.friendships.filter((f) => f.isValidated);
    friendships.forEach((friendship) => {
        if (friendship.friendWhoAsked.id === userStore.currentUser?.id) {
            friends.push(friendship.friendWhoGotAsked)
        } else {
            friends.push(friendship.friendWhoAsked)
        }
    });
    return friends;
});
</script>

<style scoped></style>
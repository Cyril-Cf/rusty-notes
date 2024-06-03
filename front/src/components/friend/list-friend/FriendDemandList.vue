<template>
    <div>
        <v-list>
            <FriendDemandItem v-for="friendship in friendRequests" :key="friendship.id.toString()"
                :friendship="friendship" />
        </v-list>
    </div>
</template>

<script lang="ts" setup>
import { computed } from 'vue';
import FriendDemandItem from './FriendDemandItem.vue';
import { useUserStore } from "@/store/userStore";

const userStore = useUserStore();

const friendRequests = computed(() => {
    return userStore.friendships.filter((f) => !f.isValidated);
});
</script>

<style scoped></style>
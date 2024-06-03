<template>
    <div class="mb-5">
        <v-btn color="primary" @click="goBackToLists">Retour à mes listes</v-btn>
    </div>
</template>

<script lang="ts" setup>
import router from "@/router";
const goBackToLists = () => {
    router.push({ path: "/my_lists" });
};

</script>

<style scoped></style>
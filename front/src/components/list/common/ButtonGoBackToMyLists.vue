<template>
    <div>
        <v-btn color="primary" @click="goBackToLists">Retour à mes listes</v-btn>
    </div>
</template>

<script lang="ts" setup>
import router from "@/router";
const goBackToLists = () => {
    router.push({ path: "/my_notes" });
};

</script>

<style scoped></style>
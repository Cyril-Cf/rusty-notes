<template>
    <h1>Bienvenue {{ $store.user.username }} !</h1>
    <button type="button" @click="$store.logout">
      Se déconnecter
    </button>
</template>

<script setup>
</script>

<style scoped></style>
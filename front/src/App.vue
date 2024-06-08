<template>
  <router-view />
</template>

<script lang="ts" setup>
import { onMounted } from 'vue';
import { useListStore } from "@/store/listStore";
onMounted(async () => {
  const listStore = useListStore();
  if (listStore.listTypes.length == 0) {
    await listStore.getAllListTypeWithAllowedItemTypes();
  }
});
</script>

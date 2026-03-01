<script setup lang="ts">
import Menubar from 'primevue/menubar';
import { ref } from 'vue';
import { useRouter } from "vue-router";

const router = useRouter();

const items = ref([
  {label: 'Back', icon: 'pi pi-angle-left', command: () => router.back()},
    { label: 'Home', icon: 'pi pi-home', command: () => router.push('/Landing-Page') },
    { label: 'Settings', icon: 'pi pi-cog', command: () => router.push('/Settings')},
    { label: 'Script Upload', icon: 'pi pi-upload', command: () => router.push('/Script-Upload')},
])
</script>

<template>
  <div class="card">
    <Menubar :model="items" />
  </div>

</template>

<style scoped>

</style>
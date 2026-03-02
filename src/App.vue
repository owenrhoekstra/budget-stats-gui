<script setup lang="ts">
import menubar from './components/menuBar.vue'
import ConfirmDialog from 'primevue/confirmdialog'

</script>

<template>
  <ConfirmDialog />
  <menubar />
  <div>
    <router-view></router-view>
  </div>
</template>

<style scoped>

</style>

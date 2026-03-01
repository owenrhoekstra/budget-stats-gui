<script setup lang="ts">
import menubar from './components/menuBar.vue'
</script>

<template>
  <menubar />
  <div>
    <h1 style="margin: 0.5rem;">Budget Stats GUI — App mounted</h1>
    <router-view></router-view>
  </div>
</template>

<style scoped>

</style>

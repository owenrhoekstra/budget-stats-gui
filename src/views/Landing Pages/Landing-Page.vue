<script setup lang="ts">
import Card from "primevue/card";
import Button from "primevue/button";
</script>

<template>
  <div class="card-container">
  <Card style="width: 25rem; overflow: hidden">
    <template #header>

      </template>
    <template #title>View Budget Data</template>
    <template #footer>
      <Button label="Enter" icon="pi pi-sign-in" @click="$router.push('/View-Data-Landing')"/>
    </template>
  </Card>
  <Card style="width: 25rem; overflow: hidden">
    <template #header>

    </template>
    <template #title>Add/Change Budget Data</template>
    <template #footer>
      <Button label="Enter" icon="pi pi-sign-in" @click="$router.push('/Change-Data-Landing')"/>
    </template>
  </Card>
  </div>
</template>

<style scoped>
.card-container {
  display: flex;
  justify-content: space-around;
}

:deep(.p-card) {
  max-width: 18rem;
}
</style>
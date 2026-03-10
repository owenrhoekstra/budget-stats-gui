<script setup lang="ts">
import Panel from "primevue/panel";
import Button from "primevue/button";
</script>

<template>
  <div class="card-container">
    <Panel header="WealthSimple">
      <Button label="Enter" icon="pi pi-sign-in" @click="$router.push('/WealthSimple-Data-View')"/>
    </Panel>
    <Panel header="Chequing">
      <Button label="Enter" icon="pi pi-sign-in" @click="$router.push('/chequing-data-view')"/>
    </Panel>
    <Panel header="Investia TFSA">
      <Button label="Enter" icon="pi pi-sign-in" @click="$router.push('/investia-TFSA-data-view')"/>
    </Panel>
  </div>
</template>

<style scoped>
.card-container {
  display: flex;
  flex-direction: row;
  gap: 3rem;
  justify-content: center;
}
</style>
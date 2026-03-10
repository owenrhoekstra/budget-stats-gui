<script setup lang="ts">
import Panel from 'primevue/panel';
import Button from 'primevue/button';
</script>

<template>
  <div class="card">
    <Panel header="Upload and View Imported Scripts">
      <p class="m-0">
        View all currently uploaded scripts and upload new scripts to the system.
        Uploaded scripts will be available for use in the specified views and can be
        selected to run.
      </p>
      <Button label="Go to Script Upload" class="mt-3" @click="$router.push('/Script-Upload')" />
    </Panel>
    <Panel header="Run Database Maintenance">
      <p class="m-0">

      </p>
    </Panel>
    <Panel header="">
    <p class="m-0">

    </p>
    </Panel>
  </div>
</template>

<style scoped>
:deep(.p-panel) {
  width: 45rem;
  margin: 0.5rem auto;
}

div {
  display: grid;
  place-items: center;
}

</style>
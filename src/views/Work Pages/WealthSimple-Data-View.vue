<script setup lang="ts">
import { useConfirm } from "primevue/useconfirm";
import Button from 'primevue/button';
import { invoke } from '@tauri-apps/api/core';

const confirm = useConfirm();

function confirmAction(scriptName: string, message: string) {
  confirm.require({
    message,
    header: "Confirmation",
    icon: "pi pi-exclamation-triangle",
    accept: () => runScript(scriptName),
  })
}

async function runScript(scriptName: string) {
  await invoke('wealthsimple_data_run_script', { scriptName });
}
</script>

<template>
  <Button
    label="Run Budget Script"
    @click="confirmAction('averages', 'Run Budget Script?')"
  />
</template>

<style scoped>

</style>
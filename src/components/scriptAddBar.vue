<script setup lang="ts">
import Menubar from "primevue/menubar";
import Dialog from "primevue/dialog";
import InputText from "primevue/inputtext";
import Textarea from "primevue/textarea";
import Button from "primevue/button";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog"

const showDialog = ref(false);
const selectedPath = ref("");
const scriptName = ref("");
const scriptDescription = ref("");

const items = ref([
  {
    label: "Choose Script",
    icon: "pi pi-file-arrow-up",
    command: () => openFilePicker()
  }
]);

async function openFilePicker() {
  try {
    const selected = await open({
      multiple: false,
      directory: false,
      filters: [{ name: 'Scripts', extensions: ['py', 'js', 'sh'] }]
    });

    if (!selected) return;

    selectedPath.value = Array.isArray(selected) ? selected[0] : selected;
    showDialog.value = true;
  } catch (err) {
    console.error('Failed to open file picker:', err);
  }
}

async function submitScript() {
  const payload = {
    path: selectedPath.value,
    name: scriptName.value,
    description: scriptDescription.value
  };

  await invoke("import_script_to_db", { payload });

  showDialog.value = false;
  scriptName.value = "";
  scriptDescription.value = "";
}
</script>

<template>
  <div class="card">
    <Menubar :model="items" />

    <Dialog v-model:visible="showDialog" modal header="Script Details" :style="{ width: '30rem' }">
      <div class="field">
        <label>Name</label>
        <InputText v-model="scriptName" class="w-full" />
      </div>

      <div class="field" style="margin-top:1rem">
        <label>Description</label>
        <Textarea v-model="scriptDescription" rows="3" class="w-full" />
      </div>

      <div style="margin-top:1.5rem; display:flex; justify-content:flex-end; gap:0.5rem">
        <Button label="Cancel" severity="secondary" @click="showDialog=false" />
        <Button label="Save" @click="submitScript" />
      </div>
    </Dialog>
  </div>
</template>

<style scoped>
:deep(.p-menubar) {
  justify-content: center;
}
</style>
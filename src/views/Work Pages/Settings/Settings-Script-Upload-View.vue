<script setup lang="ts">
import { ref, onMounted } from "vue";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import { invoke } from "@tauri-apps/api/core";
import scriptAddBar from "../../../components/scriptAddBar.vue";
import Button from "primevue/button";
import { useConfirm } from "primevue/useconfirm";
import ConfirmDialog from "primevue/confirmdialog";


interface Script {
  name: string;
  handler: string;
  description?: string;
  date_added?: string | null;
  extension?: string | null;
}

const importedScripts = ref<Script[]>([]);
const confirm = useConfirm();

onMounted(async () => {
  try {
    const scripts = await invoke<Script[]>("wealthsimple_data_get_imported_scripts");
    importedScripts.value = scripts;
  } catch (e) {
    console.error("Failed to fetch imported scripts:", e);
  }
});

const updateScripts = async () => {
  try {
    const scripts = await invoke<Script[]>("wealthsimple_data_get_imported_scripts");
    importedScripts.value = scripts;
  } catch (e) {
    console.error("Failed to fetch imported scripts:", e);
  }
}

function deleteScriptEntry(handler: string, name: string, date: string | null | undefined) {
  confirm.require({
    message: `Delete ${name} that was imported on ${date ?? "unknown date"}?`,
    header: "Confirmation",
    icon: "pi pi-exclamation-triangle",
    accept: () => {
      invoke("delete_archive_imported_scripts", { path: handler })
      .then(() => updateScripts());
    }
  });
}
</script>

<template>
  <scriptAddBar @scriptUploaded="updateScripts" />
  <DataTable :value="importedScripts">
    <Column field="name" header="Name" />
    <Column
        field="date_added"
        header="Date Added"
        :body="(row: Script) => row.date_added ?? '-'"
    />
    <Column
        field="extension"
        header="Extension"
        :body="(row: Script) => row.extension ?? '-'"
    />
    <Column header="Delete">
      <template #body="{ data }">
        <Button
          label="Delete"
          icon="pi pi-trash"
          severity="danger"
          size="small"
          @click="() => deleteScriptEntry(data.handler, data.name, data.date_added)"
        />
      </template>
    </Column>
  </DataTable>
</template>

<style scoped>

</style>
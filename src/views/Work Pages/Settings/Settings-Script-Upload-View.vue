<script setup lang="ts">
import { ref, onMounted } from "vue";
import DataTable from "primevue/datatable";
import Column from "primevue/column";
import { invoke } from "@tauri-apps/api/core";
import scriptAddBar from "../../../components/scriptAddBar.vue";

interface Script {
  name: string;
  handler: string;
  description?: string;      // optional
  date_added?: string | null; // optional, matches Rust Option
}

const importedScripts = ref<Script[]>([]);

onMounted(async () => {
  try {
    const scripts = await invoke<Script[]>("wealthsimple_data_get_imported_scripts");
    importedScripts.value = scripts;
    console.log("Loaded scripts:", scripts);
  } catch (e) {
    console.error("Failed to fetch imported scripts:", e);
  }
});
</script>

<template>
  <scriptAddBar />
  <DataTable :value="importedScripts">
    <Column field="name" header="Name" />
    <Column
        field="date_added"
        header="Date Added"
        :body="(row: Script) => row.date_added ?? '-'"
    />
  </DataTable>
</template>

<style scoped>

</style>
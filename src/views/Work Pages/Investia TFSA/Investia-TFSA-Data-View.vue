<script setup lang="ts">
import { useConfirm } from "primevue/useconfirm";
import Button from "primevue/button";
import { invoke } from "@tauri-apps/api/core";
import Card from "primevue/card";
import Carousel from "primevue/carousel";
import ProgressBar from 'primevue/progressbar';
import { ref } from "vue";
import { onMounted } from "vue";

const confirm = useConfirm();

function confirmAction_PackagedScript(scriptName: string, message: string) {
  confirm.require({
    message,
    header: "Confirmation",
    icon: "pi pi-exclamation-triangle",
    accept: () => runPackagedScript(scriptName),
  });
}

function confirmAction_ImportedScript(handler: string, message: string) {
  confirm.require({
    message,
    header: "Confirmation",
    icon: "pi pi-exclamation-triangle",
    accept: () => runImportedScript(handler),
  });
}

async function runPackagedScript(scriptName: string) {
  runningScript.value = scriptName;
  try {
    scriptOutputs.value[scriptName] = await invoke("investia_tfsa_data_run_built_in_script", { scriptName });
  } catch (err) {
    scriptOutputs.value[scriptName] = `Error: ${err}`;
  }
  runningScript.value = null;
}

async function runImportedScript(handler: string) {
  runningScript.value = handler;
  try {
    // Store output in scriptOutputs so <pre> can render it
    scriptOutputs.value[handler] = await invoke("investia_tfsa_data_run_imported_script", { handler });
  } catch (err) {
    scriptOutputs.value[handler] = `Error: ${err}`;
  }
  runningScript.value = null;
}

interface Script {
  name: string;
  description: string;
  handler: string;
}

const packagedScripts = ref<Script[]>([]);
const importedScripts = ref<Script[]>([]);
const runningScript = ref<string | null>(null);
const scriptOutputs = ref<Record<string, string>>({});

onMounted(async () => {
  try {
    packagedScripts.value = await invoke(
        "investia_tfsa_data_get_built_in_scripts",
    );
  } catch (err) {
    console.error("Error fetching built-in scripts:", err);
    packagedScripts.value = [];
  }
  try {
    importedScripts.value = await invoke(
        "investia_tfsa_data_get_imported_scripts",
    );
  } catch (err) {
    console.error("Error fetching imported scripts:", err);
    importedScripts.value = [];
  }
});
</script>

<template>
  <Carousel :value="packagedScripts" :numVisible="1" circular>
    <template #item="slotProps">
      <Card>
        <template #title>
          {{ slotProps.data.name }}
        </template>

        <template #content>
          <p>{{ slotProps.data.description }}</p>
          <Button
              label="Run"
              @click="confirmAction_PackagedScript(slotProps.data.handler, `Run ${slotProps.data.name}?`)"
              :disabled="runningScript === slotProps.data.handler"
          />
          <ProgressBar v-if="runningScript === slotProps.data.handler" mode="indeterminate" style="height:6px;margin-top:0.5rem"></ProgressBar>
          <pre v-if="scriptOutputs[slotProps.data.handler]">{{ scriptOutputs[slotProps.data.handler] }}</pre>
        </template>
      </Card>
    </template>
  </Carousel>
  <Carousel class="imported-carousel" :value="importedScripts" :numVisible="1" circular>
    <template #item="slotProps">
      <Card>
        <template #title>
          {{ slotProps.data.name }}
        </template>

        <template #content>
          <p>{{ slotProps.data.description }}</p>
          <Button
              label="Run"
              @click="confirmAction_ImportedScript(slotProps.data.handler, `Run ${slotProps.data.name}?`)"
              :disabled="runningScript === slotProps.data.handler"
          />
          <ProgressBar v-if="runningScript === slotProps.data.handler" mode="indeterminate" style="height:6px;margin-top:0.5rem"></ProgressBar>
          <pre v-if="scriptOutputs[slotProps.data.handler]">{{ scriptOutputs[slotProps.data.handler] }}</pre>
        </template>
      </Card>
    </template>
  </Carousel>
</template>

<style scoped>
.imported-carousel {
  margin-top: 2rem;
}
</style>
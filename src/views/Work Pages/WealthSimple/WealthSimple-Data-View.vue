<script setup lang="ts">
import { useConfirm } from "primevue/useconfirm";
import Button from "primevue/button";
import { invoke } from "@tauri-apps/api/core";
import Card from "primevue/card";
import Carousel from "primevue/carousel";
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
	await invoke("wealthsimple_data_run_built_in_script", { scriptName });
}

async function runImportedScript(handler: string) {
	await invoke("wealthsimple_data_run_imported_script", { handler });
}

interface Script {
	name: string;
	description: string;
	handler: string;
}

const packagedScripts = ref<Script[]>([]);
const importedScripts = ref<Script[]>([]);

onMounted(async () => {
	packagedScripts.value = await invoke(
		"wealthsimple_data_get_built_in_scripts",
	);
	importedScripts.value = await invoke(
		"wealthsimple_data_get_imported_scripts",
	);
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
          />
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
          />
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
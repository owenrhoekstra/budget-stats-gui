<script setup lang="ts">
import Menubar from "primevue/menubar";
import { ref } from "vue";
import { useRouter } from "vue-router";
import { invoke } from "@tauri-apps/api/core";

const router = useRouter();

const items = ref([
	{ label: "Back", icon: "pi pi-angle-left", command: () => router.back() },
	{
		label: "Home",
		icon: "pi pi-home",
		command: () => router.push("/Landing-Page"),
	},
	{
		label: "Download Budget Data",
		icon: "pi pi-download",
		command: async () => await invoke("csv_import_apple_script"),
	},
	{
		label: "Settings",
		icon: "pi pi-cog",
		command: () => router.push("/Settings"),
	},
	{
		label: "Script Upload",
		icon: "pi pi-upload",
		command: () => router.push("/Script-Upload"),
	},
]);
</script>

<template>
  <div class="card">
    <Menubar :model="items" />
  </div>

</template>

<style scoped>
:deep(.p-menubar) {
  justify-content: center;
}
</style>
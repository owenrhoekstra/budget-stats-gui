<script setup lang="ts">
import { ref } from 'vue';
import { useToast } from 'primevue/usetoast';
import { useRouter } from "vue-router";
import SpeedDial from 'primevue/speeddial';
import Toast from 'primevue/toast';

const toast = useToast();
const router = useRouter();

const items = ref([
  {
    label: 'Add',
    icon: 'pi pi-pencil',
    command: () => {
      toast.add({ severity: 'info', summary: 'Add', detail: 'Data Added', life: 3000 });
    }
  },
  {
    label: 'Update',
    icon: 'pi pi-refresh',
    command: () => {
      toast.add({ severity: 'success', summary: 'Update', detail: 'Data Updated', life: 3000 });
    }
  },
  {
    label: 'Delete',
    icon: 'pi pi-trash',
    command: () => {
      toast.add({ severity: 'error', summary: 'Delete', detail: 'Data Deleted', life: 3000 });
    }
  },
  {
    label: 'Upload',
    icon: 'pi pi-upload',
    command: () => {
      router.push('/fileupload');
    }
  },
  {
    label: 'Vue Website',
    icon: 'pi pi-external-link',
    command: () => {
      window.location.href = 'https://vuejs.org/'
    }
  }
])
</script>

<template>
  <div class="card">
    <div :style="{ position: 'relative', height: '500px' }" class="flex items-center justify-center">
      <SpeedDial :model="items" :radius="80" type="circle" :style="{ position: 'absolute' }" :buttonProps="{ severity: 'warn', rounded: true }" />
      <Toast />
    </div>
  </div>
</template>

<style scoped>

</style>
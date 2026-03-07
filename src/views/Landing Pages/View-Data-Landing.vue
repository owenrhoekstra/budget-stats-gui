<script setup lang="ts">
import Card from "primevue/card";
import Button from "primevue/button";
import Carousel from "primevue/carousel";
import Panel from "primevue/panel";
import { ref, onMounted } from "vue";
import { getAlerts } from "../../controller/view-data-landing-carousel.ts";

const alerts = ref<any[]>([]);

onMounted(async () => {
	alerts.value = await getAlerts();
});
</script>

<template>
<div class="card-container">
  <Card style="width: 25rem; overflow: hidden">
    <template #header>

    </template>
    <template #title>Account Values</template>
    <template #footer>
      <Button label="Enter" icon="pi pi-sign-in" @click="$router.push('/Data-View-Account-Selection')"/>
    </template>
  </Card>
  <Card style="width: 25rem; overflow: hidden">
    <template #header>

    </template>
    <template #title>Transaction Logs</template>
    <template #footer>
      <Button label="Enter" icon="pi pi-sign-in" @click="$router.push('/develop')"/>
    </template>
  </Card>
  <Card style="width: 25rem; overflow: hidden">
    <template #header>

    </template>
    <template #title>Spending Habits</template>
    <template #footer>
      <Button label="Enter" icon="pi pi-sign-in" @click="$router.push('/develop')"/>
    </template>
  </Card>
</div>
  <Carousel :value="alerts" :numVisible="3" :numScroll="1" circular :autoplayInterval="4000">
    <template #item="slotProps">
      <Panel class="m-1">
        <template #header>
          <div class="text-lg font-semibold alert-title">
            {{ slotProps.data.title }}
          </div>
        </template>

        <div class="alert-messages text-sm text-center p-3">
          <div>
            {{ slotProps.data.message }}
          </div>
          <div>
            {{ slotProps.data.message2 }}
          </div>
        </div>
      </Panel>
    </template>
  </Carousel>
</template>

<style scoped>
.card-container {
  display: flex;
  justify-content: space-around;
  gap: 2rem;
  margin-bottom: 5rem;
}

:deep(.p-card) {
  max-width: 18rem;
}

.alert-messages {
  display: flex;
  flex-direction: column;
  gap: 0.3rem;
  padding-bottom: 1.75rem;
  justify-content: center;
}

:deep(.p-carousel-item) {
  display: flex;
  justify-content: center;
}

:deep(.p-carousel) {
  max-width: 500px;
  min-width: 500px;
}

.alert-title {
  margin-top: 0.75rem;
  text-align: center;
  font-weight: 700;
}

:deep(.p-panel-header) {
  display: flex;
  justify-content: center;
}

:deep(.p-panel) {
  max-width: 500px;
  min-width: 300px;
  max-height: 140px;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

:deep(.p-panel-content) {
  padding: 0.2rem;
  display: flex;
  flex-direction: column;
  justify-content: center;
}
</style>
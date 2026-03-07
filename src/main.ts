import "./style.css";
import PrimeVue from "primevue/config";
import ToastService from "primevue/toastservice";
import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import "primeicons/primeicons.css";
import "primeicons/primeicons.css";
import ConfirmationService from "primevue/confirmationservice";
import Aura from "@primeuix/themes/aura";

const app = createApp(App);

app.use(router);
app.use(ToastService);
app.use(ConfirmationService);

app.use(PrimeVue, {
	theme: {
		preset: Aura,
		options: {
			prefix: "p",
			darkModeSelector: "system",
			cssLayer: false,
		},
	},
});

app.mount("#app");

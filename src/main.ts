import './style.css'
import PrimeVue from 'primevue/config'
import ToastService from 'primevue/toastservice'
import Aura from '@primeuix/themes/aura'
import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import 'primeicons/primeicons.css'
import 'primeicons/primeicons.css'
import ConfirmationService from 'primevue/confirmationservice';

const app = createApp(App)

app.use(router)
app.use(ToastService)
app.use(ConfirmationService)

app.use(PrimeVue, {
    theme: {
        preset: Aura
    }
})

app.mount('#app')

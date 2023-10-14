import App from "./App.vue";
import router from "./router";
import { createApp } from "vue";
import { createPinia } from "pinia";
import PrimeVue from "primevue/config";

import "primevue/resources/themes/lara-light-indigo/theme.css";
import "primevue/resources/primevue.min.css";
import "./style.css";

const app = createApp(App);

app.use(PrimeVue).use(createPinia()).use(router);

app.mount("#app");

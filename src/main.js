import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import store from "./store";
import mitt from "mitt";

const bus = mitt();
const app = createApp(App).use(store).use(router);

app.config.globalProperties.$bus = bus;
app.mount("#app");

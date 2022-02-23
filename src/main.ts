import { createApp } from "vue";
import App from "./App.vue";
import Router from "./router";
import Store from "./store";

import "./index.css";

createApp(App).use(Store).use(Router).mount("#app");

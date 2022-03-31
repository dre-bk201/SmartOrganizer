import { createApp } from "vue";
import App from "./App.vue";
import Router from "./router";
import Store from "./store";
import Toast, { PluginOptions, POSITION } from "vue-toastification";
import "vue-toastification/dist/index.css";

import "./index.css";

const app = createApp(App);

const options: PluginOptions = {
  position: POSITION.TOP_LEFT,
  timeout: 3000,
  closeOnClick: true,
  pauseOnFocusLoss: true,
  pauseOnHover: true,
  draggable: true,
  draggablePercent: 0.6,
  showCloseButtonOnHover: false,
  hideProgressBar: true,
  closeButton: "button",
  icon: true,
  rtl: false,
};
app.use(Store).use(Toast, options).use(Router).mount("#app");

import { createRouter, createWebHashHistory } from "vue-router";
import Dashboard from "../views/Dashboard.vue";
import Statistics from "../views/Statistics.vue";
import Logs from "../views/Logs.vue";

const routes = [
  {
    path: "/",
    name: "Dashboard",
    component: Dashboard,
  },
  {
    path: "/statistics",
    name: "Statistics",
    component: Statistics,
  },
  {
    path: "/logs",
    name: "Logs",
    component: Logs,
  },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});

export default router;

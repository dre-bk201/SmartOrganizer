import { createRouter, createWebHistory } from "vue-router";

import Dashboard from "../pages/Dashboard.vue";
import Journal from "../pages/Journal.vue";
import Statistics from "../pages/Statistics.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/",
      name: "Dashboard",
      component: Dashboard,
    },
    {
      path: "/journal",
      name: "Journal",
      component: Journal,
    },
    {
      path: "/statistics",
      name: "Statistics",
      component: Statistics,
    },
  ],
});

export default router;

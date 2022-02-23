import { createWebHistory, createRouter } from "vue-router";

import Dashboard from "../views/Dashboard.vue";
import Journal from "../views/Journal.vue";
import Statistics from "../views/Statistics.vue";

import Action from "../components/routes/Action.vue";
import Monitor from "../components/routes/Monitor.vue";
import Rules from "../components/routes/Rules.vue";

export default createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      component: Dashboard,
      name: "Dashboard",

      children: [
        { path: "/monitor", component: Monitor, name: "Monitor" },
        { path: "/actions", component: Action, name: "Action" },
        { path: "/rules", component: Rules, name: "Rules" },
      ],
    },

    { path: "/statistics", component: Statistics, name: "Statistics" },
    { path: "/journal", component: Journal, name: "Journal" },
  ],
});

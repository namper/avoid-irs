import Vue from "vue";
import VueRouter, { RouteConfig } from "vue-router";
import IncometTab from "@/components/IncometTab.vue";

Vue.use(VueRouter);

const routes: Array<RouteConfig> = [
  {
    path: "/",
    name: "Home",
    component: IncometTab,
  },
];

const router = new VueRouter({
  routes,
});

export default router;

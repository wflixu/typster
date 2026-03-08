import { RouteRecordRaw, createRouter, createWebHashHistory } from "vue-router";
import Home from "./pages/home/Home.vue";
import Settings from "./pages/settings/Settings.vue";

const routes = [
  { path: "/", redirect: "/home" },
  {
    path: "/home",
    component: Home,
  },
  {
    path: "/settings",
    name: "Settings",
    component: Settings,
    meta: { title: "设置" }
  }
];

const router = createRouter({
  // 4. Provide the history implementation to use. We are using the hash history for simplicity here.
  history: createWebHashHistory(),
  routes, // short for `routes: routes`
});

export { router };

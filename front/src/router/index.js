import { createWebHistory, createRouter } from "vue-router";
import { useAuthStore } from "@/stores/authStore";

import Home from "@views/HomeView.vue";
import MyNotes from "@views/MyNotesView.vue";
import PageNotFoundView from "@/views/PageNotFoundView.vue";

const routes = [
  {
    path: "/my-notes",
    name: "my-notes",
    meta: { requiresAuth: false },
    component: MyNotes,
  },
  {
    path: "/",
    name: "home",
    meta: { requiresAuth: false },
    component: Home,
  },
  { path: "/:pathMatch(.*)*", name: "NotFound", component: PageNotFoundView },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

router.beforeEach((to, from, next) => {
  const authStore = useAuthStore();
  const isAuthenticated = authStore.authenticated;
  if (to.meta.requiresAuth && !isAuthenticated) {
    next({ name: "home" });
  } else {
    next();
  }
});

export default router;

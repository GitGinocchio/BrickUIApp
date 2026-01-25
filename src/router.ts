import { invoke } from '@tauri-apps/api/core'

import { createRouter, createWebHashHistory } from 'vue-router'
import BricksPage from './views/BricksPage.vue'
import SettingsPage from './views/SettingsPage.vue'
import Marketplace from './views/Marketplace.vue'
import BrickPage from './views/BrickPage.vue'
import GetStarted from './views/GetStarted.vue'
import User from './views/User.vue'

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/bricks' },
    { path: '/bricks', component: BricksPage },
    { path: '/brick/:name', props: true, component: BrickPage },
    { path: '/settings', component: SettingsPage },
    { path: '/marketplace', component: Marketplace, meta: { requiresAuth: true } },
    { path: '/get-started', component: GetStarted },
    { path: '/user', component: User, meta: { requiresAuth: true } }
  ]
});

router.beforeEach(async (to, _from, next) => {
  const is_logged_in: boolean = await invoke("auth_is_logged_in");

  if (to.path == '/get-started' && is_logged_in) {
    next('/user')
  }
  else if (to.meta.requiresAuth && !is_logged_in) {
    next('/get-started');
  }
  else {
    next();
  }
});
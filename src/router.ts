import { createRouter, createWebHashHistory } from 'vue-router'
import BricksManager from './views/BricksManager.vue'
import SettingsPage from './views/SettingsPage.vue'
import Marketplace from './views/Marketplace.vue'

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/widgets' },
    { path: '/widgets', component: BricksManager },
    { path: '/settings', component: SettingsPage },
    { path: '/marketplace', component: Marketplace }
  ]
})

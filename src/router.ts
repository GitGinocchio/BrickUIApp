import { createRouter, createWebHashHistory } from 'vue-router'
import WidgetManager from './views/WidgetManager.vue'
import SettingsPage from './views/SettingsPage.vue'
import Marketplace from './views/Marketplace.vue'

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/widgets' },
    { path: '/widgets', component: WidgetManager },
    { path: '/settings', component: SettingsPage },
    { path: '/marketplace', component: Marketplace }
  ]
})

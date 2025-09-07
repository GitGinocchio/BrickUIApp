import { createRouter, createWebHashHistory } from 'vue-router'
import BricksManager from './views/BricksManager.vue'
import SettingsPage from './views/SettingsPage.vue'
import Marketplace from './views/Marketplace.vue'
import BrickPage from './views/BrickPage.vue'

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/bricks' },
    { path: '/bricks', component: BricksManager },
    { path: '/brick/:name', props: true, component: BrickPage },
    { path: '/settings', component: SettingsPage },
    { path: '/marketplace', component: Marketplace }
  ]
})

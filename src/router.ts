import { createRouter, createWebHashHistory } from 'vue-router'
import BricksPage from './views/BricksPage.vue'
import SettingsPage from './views/SettingsPage.vue'
import Marketplace from './views/Marketplace.vue'
import BrickPage from './views/BrickPage.vue'

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: '/', redirect: '/bricks' },
    { path: '/bricks', component: BricksPage },
    { path: '/brick/:name', props: true, component: BrickPage },
    { path: '/settings', component: SettingsPage },
    { path: '/marketplace', component: Marketplace }
  ]
})

import { createMemoryHistory, createRouter } from 'vue-router'

const routes = [
  { path: '/', component: () => import('../pages/HomeView.vue') },
  { path: '/texture', component: () => import('../pages/Texture.vue') }
]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})
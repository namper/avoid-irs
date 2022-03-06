import { createRouter, createWebHistory } from 'vue-router'
import TaxView from '../views/TaxView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: TaxView 
    }
  ]
})

export default router

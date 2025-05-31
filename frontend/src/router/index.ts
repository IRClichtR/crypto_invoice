import { createRouter, createWebHistory } from 'vue-router'
import HomePage from '../views/HomePage.vue'
import WelcomePage from '../views/WelcomePage.vue' // Ensure this file exists in the specified path

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomePage,
      meta: {
        title: 'Leka - Home',
        description: 'Welcome to Leka your favorite payment solution',
        requestedAuth: false // No auth needed for this page
      }
    },

    {
      path: '/Welcome',
      name: 'welcome',
      component: WelcomePage,
      meta: {
        title: 'Leka - Welcome page',
        description: 'Welcome to Leka, your favorite payment solution',
        requiresAuth: true // Protected view
      }
    },
    
    // // Invoices 
    // {
    //   path: '/invoices',
    //   name: 'invoices', 
    //   component: () => import('@/views/Invoices.vue'),
    //   meta: {
    //     title: 'Leka - Factures',
    //     description: 'Gestion des factures',
    //     requiresAuth: true
    //   }
    // },
    
    // // User params
    // {
    //   path: '/settings',
    //   name: 'settings',
    //   component: () => import('@/views/Settings.vue'),
    //   meta: {
    //     title: 'Leka - Paramètres',
    //     description: 'Paramètres utilisateur',
    //     requiresAuth: true
    //   }
    // },
    
    // // Not found
    // {
    //   path: '/:pathMatch(.*)*',
    //   name: 'not-found',
    //   component: () => import('@/views/NotFound.vue'),
    //   meta: {
    //     title: 'Page non trouvée - Leka'
    //   }
    // }
  ],
})

export default router

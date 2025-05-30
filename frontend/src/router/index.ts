import { createRouter, createWebHistory } from 'vue-router'
import HomePage from '../views/HomePage.vue'

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
      path: '/about',
      name: 'about',
      // route level code-splitting
      // this generates a separate chunk (About.[hash].js) for this route
      // which is lazy-loaded when the route is visited.
      component: () => import('../views/AboutView.vue'),
    },
     // Dashboard of the user 
    // {
    //   path: '/dashboard',
    //   name: 'dashboard',
    //   // Chargement paresseux (lazy loading) pour optimiser les performances
    //   component: () => import('@/views/Dashboard.vue'),
    //   meta: {
    //     title: 'Leka - Tableau de bord',
    //     description: 'Tableau de bord utilisateur',
    //     requiresAuth: true // Cette page nécessitera une authentification
    //   }
    // },
    
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

<script setup lang="ts">  
import { computed } from 'vue';
import { useRoute } from 'vue-router';

const route  = useRoute()

const shouldShowNavigation = computed(() => {
    // Show navigation if the current route is not the home page
    return route.name !== 'home';
})

const shouldShowFooter = computed(() => {
    // Show footer if the current route is not the home page
    return route.name !== 'home';
})

console.log('Leka - App component loaded 🚀')
console.log('Model: ', import.meta.env.MODE);
console.log('Base URL: ', import.meta.env.BASE_URL);

</script>

<template>
    <div id="app">
    <!-- Zone de navigation (à développer plus tard) -->
    <nav v-if="shouldShowNavigation" class="main-navigation">
      <div class="nav-container">
        <router-link to="/" class="nav-brand">Leka</router-link>
        
        <div class="nav-links">
          <router-link to="/dashboard">Dashboard</router-link>
          <router-link to="/invoices">Invoices</router-link>
          <router-link to="/settings">Settings</router-link>
        </div>
      </div>
    </nav>

    <main class="main-content">
      <router-view />
    </main>

    <footer v-if="shouldShowFooter" class="main-footer">
      <p>&copy; 2025 Leka - En cours de developpement </p>
    </footer>
  </div>
</template>

<style>
/*
 * Styles globaux pour l'application Leka
 *
 * Cette section contient uniquement les styles spécifiques à la structure
 * de l'application (layout, navigation, etc.). La configuration des polices
 * et les styles de base sont gérés dans main.css pour éviter les conflits.
 */

/* 
 * Reset CSS minimal pour assurer la cohérence entre navigateurs
 * Cette section établit une base propre pour tous les éléments
 */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

/* 
 * Configuration de base pour html et body
 * Notez l'absence de font-family ici - cette responsabilité appartient
 * maintenant à main.css qui gère notre configuration de police monospace
 */
html, body {
  height: 100%;
  line-height: 1.6;
  color: #2c3e50;
  background-color: #f8f9fa;
  /* La police monospace est appliquée via les variables CSS dans main.css */
}

/* 
 * Conteneur principal de l'application
 * Cette configuration utilise Flexbox pour créer une mise en page
 * qui s'étend sur toute la hauteur de l'écran, avec le contenu
 * principal qui prend tout l'espace disponible entre la navigation et le footer
 */
#app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

/* 
 * Styles pour la barre de navigation principale
 * Cette navigation utilise une apparence moderne avec ombre portée
 * et s'adapte parfaitement à notre police monospace
 */
.main-navigation {
  background: white;
  border-bottom: 1px solid #e9ecef;
  padding: 1rem 0;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
  /* La police monospace donnera à la navigation un aspect technique distinctif */
}

/* 
 * Conteneur interne de la navigation
 * Cette approche center-constrained assure que le contenu reste
 * lisible sur les grands écrans tout en utilisant efficacement l'espace
 */
.nav-container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 1rem;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

/* 
 * Logo/marque de l'application dans la navigation
 * L'espacement de lettres supplémentaire tire parti des caractéristiques
 * de notre police monospace pour créer un effet visuel distinctif
 */
.nav-brand {
  font-size: 1.5rem;
  font-weight: 700;
  color: #ff6b35;
  text-decoration: none;
  transition: color 0.2s ease;
  letter-spacing: 0.05em; /* Optimisé pour les polices monospace */
}

.nav-brand:hover {
  color: #e55a2b;
}

/* 
 * Conteneur pour les liens de navigation
 * L'espacement généreux améliore la lisibilité avec les polices monospace
 */
.nav-links {
  display: flex;
  gap: 2rem;
}

/* 
 * Styles individuels pour chaque lien de navigation
 * L'espacement de lettres subtil améliore la lisibilité des polices monospace
 * dans le contexte de navigation où la clarté est primordiale
 */
.nav-links a {
  color: #495057;
  text-decoration: none;
  font-weight: 500;
  transition: color 0.2s ease;
  padding: 0.5rem 0;
  position: relative;
  letter-spacing: 0.025em; /* Améliore la lisibilité avec Fira Code */
}

.nav-links a:hover,
.nav-links a.router-link-active {
  color: #ff6b35;
}

/* 
 * Indicateur visuel pour le lien actif
 * Cette ligne colorée sous le lien actif utilise la même couleur
 * que votre palette pour maintenir la cohérence visuelle
 */
.nav-links a.router-link-active::after {
  content: '';
  position: absolute;
  bottom: -1rem;
  left: 0;
  right: 0;
  height: 2px;
  background: #ff6b35;
}

/* 
 * Zone de contenu principal
 * Cette configuration permet au contenu de prendre tout l'espace
 * disponible, poussant le footer vers le bas de la page
 */
.main-content {
  flex: 1;
  width: 100%;
}

/* 
 * Footer de l'application
 * Design sobre qui complète l'esthétique technique de votre application
 * sans détourner l'attention du contenu principal
 */
.main-footer {
  background: #2c3e50;
  color: white;
  text-align: center;
  padding: 1rem;
  font-size: 0.9rem;
  /* La police monospace donnera au footer un aspect cohérent avec le reste */
}

/* 
 * Classe utilitaire pour les liens exacts actifs
 * Cette règle assure la cohérence des couleurs dans toute l'application
 */
.router-link-exact-active {
  color: #ff6b35;
}

/* 
 * Adaptations pour les écrans de taille moyenne et petite
 * Ces règles assurent que votre application reste utilisable et élégante
 * sur tous les appareils, des smartphones aux tablettes
 */
@media (max-width: 768px) {
  .nav-container {
    flex-direction: column;
    gap: 1rem;
  }
  
  .nav-links {
    gap: 1rem;
  }
  
  .nav-links a {
    font-size: 0.9rem;
  }
}

/* 
 * Classes utilitaires pour le développement et le débogage
 * Ces classes vous aident à visualiser les limites des éléments
 * pendant le développement - très utiles pour diagnostiquer
 * les problèmes de mise en page
 */
.debug-border {
  border: 1px solid red !important;
}

.debug-background {
  background-color: rgba(255, 0, 0, 0.1) !important;
}
</style>
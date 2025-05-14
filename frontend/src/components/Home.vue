<template>
    <div class="home">
      <h1>{{ title }}</h1>
      <p>Bienvenue sur votre application Vue.js servie par Axum!</p>
      <div v-if="configLoaded" class="config-info">
        <p>Configuration chargée avec succès!</p>
        <p v-if="isDebugMode">Mode debug activé</p>
      </div>
    </div>
  </template>
  
  <script>
  export default {
    name: 'HomeView',
    data() {
      return {
        title: 'Page d\'accueil',
        configLoaded: false,
        isDebugMode: false,
        apiUrl: ''
      }
    },
    mounted() {
      // Vérifier si la configuration backend est disponible
      if (window.BACKEND_CONFIG) {
        this.configLoaded = true;
        this.isDebugMode = window.BACKEND_CONFIG.debug;
        this.apiUrl = window.BACKEND_CONFIG.api_url;
        console.log('Configuration du backend chargée:', window.BACKEND_CONFIG);
      } else {
        console.warn('Configuration du backend non disponible');
      }
    }
  }
  </script>
  
  <style scoped>
  .home {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    font-family: 'Arial', sans-serif;
  }
  
  h1 {
    color: #2c3e50;
    margin-bottom: 20px;
  }
  
  p {
    line-height: 1.6;
    margin-bottom: 15px;
  }
  
  .config-info {
    margin-top: 30px;
    padding: 15px;
    background-color: #f0f4f8;
    border-radius: 5px;
    border-left: 4px solid #4299e1;
  }
  </style>
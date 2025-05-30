import { createApp } from "vue";
import { createPinia } from "pinia";
import App from "./App.vue";
import router from "./router";


const app = createApp(App);

const pinia = createPinia();
app.use(pinia);

app.use(router);

if (import.meta.env.DEV) {
    console.log('🚀 Leka - Démarrage en mode développement')
    console.log('🔧 Configuration:', {
      apiUrl: import.meta.env.VITE_API_BASE_URL,
      appName: import.meta.env.VITE_APP_NAME,
      debugMode: import.meta.env.VITE_DEBUG_MODE
    })
    
    app.config.performance = true
}

app.config.errorHandler = (err, instance, info) => {
    console.error('❌ Erreur globale de l\'application:', err)
    console.error('📍 Contexte:', info)
    
    // En production, vous pourriez envoyer ces erreurs à un service de monitoring
    if (import.meta.env.PROD) {
      // Exemple : Sentry, LogRocket, etc.
      // sendErrorToMonitoring(err, instance, info)
    }
}

app.config.globalProperties.$appName = import.meta.env.VITE_APP_NAME || 'Leka';
app.config.globalProperties.$version = '1.0.0';

app.mount('#app');

console.log(`🌟 Leka - Application ${app.config.globalProperties.$appName} est prête !`)
console.log(`📦 Version: ${app.config.globalProperties.$version}`)
console.log(`🌐 API Base URL: ${import.meta.env.VITE_API_BASE_URL}`);
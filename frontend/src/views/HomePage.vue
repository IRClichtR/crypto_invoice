<script setup lang="ts">
    import { ref } from 'vue';

    const isLoading = ref(false)
    const statusMessage = ref('')

    // click on  login button
    const handleEthereumLogin = () => {
        // Activate state to show loading
        isLoading.value = true;
        statusMessage.value = 'Connecting to Ethereum...';

        // Simulate a connexion attempt
        setTimeout(() => {
            // Simulate a successful connection
            alert('You pressed the login button!');
            
            // Reinitialize the loading state and status message
            isLoading.value = false;
            statusMessage.value = 'Connected to Ethereum successfully!';
        }, 1000);

    }

    console.log('Leka - HomePage component loaded 🚀');
</script>

<template>
    <div class="home-container">
        <header class="hero-section">
            <h1 class="app-title">Leka</h1>
            <p class="app-description">
                Create and pay invoices on the Ethereum blockchain.
                <span class="highlight-tech">Decentralized • Transparent • Private</span>
            </p>
        </header>

        <!-- Auth section-->
         <main class="auth-section">
            <div class="login-container">
                <p class="login-prompt">Please connect your Ethereum wallet to continue:</p>
                <button 
                    class="ethereum-login-btn" 
                    @click="handleEthereumLogin" 
                    :disabled="isLoading"
                >
                <span v-if="isLoading">Connecting...</span>
                <span v-else>Connect with Ethereum</span>
                </button>

                <!-- DEBUG   -->
                <div class="status-message" v-if="statusMessage">
                    {{ statusMessage }}
                </div>
            </div>
        </main>
    </div>
</template>

<style scoped>
/* Styles avec police monospace pour un look technique */

.home-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
  text-align: center;
  /* La police monospace est maintenant définie globalement dans main.css */
}

/* Section héro avec titre principal */
.hero-section {
  margin-bottom: 3rem;
}

.app-title {
  font-size: 3rem;
  color: #2c3e50;
  margin-bottom: 1rem;
  font-weight: 700;
  letter-spacing: 0.05em; /* Espacement augmenté pour l'effet monospace */
}

.app-description {
  font-size: 1.2rem;
  color: #666;
  line-height: 1.6;
  max-width: 600px;
  margin: 0 auto;
  letter-spacing: 0.025em; /* Légère amélioration de la lisibilité */
}

/* Style spécial pour les mots-clés techniques */
.highlight-tech {
  color: #ff6b35;
  font-weight: 600;
  font-family: var(--font-mono);
  background: rgba(255, 107, 53, 0.1);
  padding: 0.25rem 0.5rem;
  border-radius: 4px;
  display: inline-block;
  margin-top: 0.5rem;
  border: 1px solid rgba(255, 107, 53, 0.2);
}

/* Section d'authentification */
.auth-section {
  margin-bottom: 4rem;
}

.login-container {
  background: #f8f9fa;
  padding: 2rem;
  border-radius: 12px;
  border: 1px solid #e9ecef;
}

.login-prompt {
  font-size: 1.1rem;
  color: #495057;
  margin-bottom: 1.5rem;
}

/* Bouton de connexion Ethereum */
.ethereum-login-btn {
  background: #ff6b35;
  color: white;
  border: none;
  padding: 1rem 2rem;
  font-size: 1.1rem;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
  font-weight: 600;
}

.ethereum-login-btn:hover:not(:disabled) {
  background: #e55a2b;
  transform: translateY(-2px);
}

.ethereum-login-btn:disabled {
  background: #ccc;
  cursor: not-allowed;
  transform: none;
}

/* Message de statut */
.status-message {
  margin-top: 1rem;
  padding: 0.5rem;
  background: #e3f2fd;
  color: #1976d2;
  border-radius: 4px;
  font-size: 0.9rem;
}

/* Section des fonctionnalités */
.features-preview {
  margin-top: 3rem;
}

.features-preview h2 {
  color: #2c3e50;
  margin-bottom: 2rem;
}

.features-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 1.5rem;
  margin-top: 2rem;
}

.feature-card {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  border: 1px solid #e9ecef;
  transition: transform 0.2s ease;
}

.feature-card:hover {
  transform: translateY(-4px);
}

.feature-card h3 {
  color: #2c3e50;
  margin-bottom: 0.5rem;
  font-size: 1.1rem;
}

.feature-card p {
  color: #666;
  font-size: 0.9rem;
  line-height: 1.4;
}

/* Responsive design */
@media (max-width: 768px) {
  .home-container {
    padding: 1rem;
  }
  
  .app-title {
    font-size: 2rem;
  }
  
  .features-grid {
    grid-template-columns: 1fr;
  }
}
</style>
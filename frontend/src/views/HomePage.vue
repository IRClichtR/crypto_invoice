<script setup lang="ts">
    import { onMounted, ref, watch } from 'vue';
    import { computed } from 'vue';
    import { useRouter } from 'vue-router';
    import { useAuthStore } from '../stores/useAuthStore';
  
    // Import the auth store
    const authStore = useAuthStore();
    const router = useRouter();

  // States
    // Reactive variables for login state and status message
    const isAuthenticated = computed(() => authStore.isAuthenticated);
    const isConnecting = computed(() => authStore.isConnecting);
    const hasError = computed(() => authStore.hasError);  

  // contextual messages for user
    const statusMessage = computed(() => {
      switch (authStore.authState) {
        case 'connecting':
          return 'Connecting to Ethereum...';
        case 'challenging':
          return 'Challenge in progress...';
        case 'signing':
          return 'Signing the challenge...';
        case 'authenticating':
          return 'Authenticating...';
        case 'authenticated':
          return 'Authentication successful!';
        case 'error':
          return 'An error occurred while connecting to Ethereum.';
        default:
          return '';
      }
    })

    // button text based on connection state
    const buttonText = computed(() => {
    if (isConnecting.value) {
        switch (authStore.authState) {
          case 'connecting':
            return 'Connecting...'
          case 'challenging':
            return 'Prepare...'
          case 'signing':
            return 'Signature...'
          case 'authenticating':
            return 'Verification...'
          default:
            return 'Connecting...'
        }
      }
      
      if (hasError.value) {
        return 'Retry';
      }
      
      return 'Connect with MetaMask'
   })

   const isMetaMaskAvailable = computed(() => authStore.checkMetaMaskAvailability());

   // User actions
   async function handleEthereumLogin(): Promise<void> {
    try {
      authStore.clearError(); // Clear any previous errors

      await authStore.authenticateWithEther();

      if (authStore.isAuthenticated) {
        // Redirect to the  or another page after successful authentication
        router.push('/');
      }
      setTimeout(() => {
        // Reinitialize the loading state and status message
        authStore.resetAuthState();
      }, 1000);
    } catch(error) {
      console.error('Authentication error:', error);
    }
  }

    /**
   * Gestion du guide d'installation MetaMask
   */
  function handleInstallMetaMask(): void {
      const metamaskUrl = 'https://metamask.io/download/'
      window.open(metamaskUrl, '_blank', 'noopener,noreferrer')
  } 

    /**
   * retry
   */
  function handleRetry(): void {
    authStore.clearError()
  }

  /**
   * Hooks de cycle de vie
   */

  /**
   * Initialisation du composant
   * 
   * Au montage du composant, nous initialisons l'état d'authentification.
   * Cela permet de restaurer automatiquement une session utilisateur existante
   * si des tokens valides sont trouvés dans le localStorage.
   */
  onMounted(() => {
    console.log('🏠 HomePage initialization')
    
    // Initialise l'état d'authentification (récupère les tokens stockés)
    authStore.initializeAuth()
    
    // Si l'utilisateur est déjà connecté, le redirige automatiquement
    if (authStore.isAuthenticated) {
      console.log('👤 User already connected, Redirection')
      router.push('/welcome')
    }
  })

  watch(
    () => authStore.isAuthenticated,
    (newValue) => {
      if (newValue) {
        console.log('User is authenticated:', authStore.user);
        setTimeout(() => {
          // Reinitialize the loading state and status message
          authStore.resetAuthState(); // Replace with a valid method
          router.push('/welcome');
        }, 1000);
        // Redirect to the  or another page after successful authentication
      }
    }
  )

  console.log(
    '🏠 HomePage component initialized. Authenticated:', 
    isAuthenticated.value, 
    'Connecting:', 
    isConnecting.value, 
    'Error:', 
    hasError.value
  )
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
            <!-- If Metamask is not available connect with metamask -->
            <div v-if="!isMetaMaskAvailable" class="metamask-warning">
              <div class="warning-icon">⚠️</div>
              <p>
                MetaMask is not installed. Please install it to use Leka.
                <button @click="handleInstallMetaMask" class="install-metamask-btn">Install MetaMask</button>
              </p>
            </div>
            <!-- if user has metamask in his browser -->
            <div v-else>
              <p class="login-prompt">
                Connect your Ethereum wallet to start using Leka.
              </p>
              <button 
                class="ethereum-login-btn" 
                @click="handleEthereumLogin" 
                :disabled="isConnecting || isAuthenticated"
              >
                {{ buttonText }}
              </button>
          </div>
          </div>  

          <!-- <button
          class="ethereum-login-btn"
          :class="{
            'loading': isConnecting,
            'error': hasError,
            'success': isAuthenticated
          }"
          @click="hasError ? handleRetry() : handleEthereumLogin()"
          :disabled="isConnecting || isAuthenticated"
          >
          Affiche le texte du bouton en fonction de l'état de connexion
          <span class="btn-icon">
              <span v-if="isConnecting" class="loading-spinner"></span>
              <span v-else-if="hasError" class="error-icon">❌</span>
              <span v-else-if="isAuthenticated" class="success-icon">✅</span>
              <span v-else class="default-icon">🦊</span>
          </span>
          <span class="btn-text">{{ buttonText }}</span>
        </button> -->

        <!-- Detailled status message -->
         <div v-if="statusMessage" class="status-container">
          <div
            class="status-message"
            :class="{
              'status-loading': isConnecting,
              'status-error': hasError,
              'status-success': isAuthenticated
            }"
          >
            {{ statusMessage }}
          </div> 

          <!-- Progess indicator -->
          <div v-if="isConnecting" class="progress-indicator">
            <div class="progress-bar">
              <div 
                class="progress-fill" 
                :class="`step-${authStore.authState}`"
              ></div>
            </div>
              <div class="progress-steps">
                <span :class="{ active: ['connecting', 'challenging', 'signing', 'authenticating'].includes(authStore.authState) }">Connexion</span>
                <span :class="{ active: ['challenging', 'signing', 'authenticating'].includes(authStore.authState) }">Challenge</span>
                <span :class="{ active: ['signing', 'authenticating'].includes(authStore.authState) }">Signature</span>
                <span :class="{ active: authStore.authState === 'authenticating' }">Vérification</span>
            </div>
          </div>

          <!-- Aide contextuelle pour les nouveaux utilisateurs -->
          <div v-if="!isConnecting && !isAuthenticated && !hasError" class="help-section">
            <details class="help-details">
              <summary>First connexion to Leka? Click here to get help</summary>
              <div class="help-content">
                <h4>How to connect?</h4>
                <ol>
                  <li>Click on</li>
                  <li>MetaMask will open automatically</li>
                  <li>Choose the account you want to use</li>
                  <li>Sign the security message to prove who you are</li>
                  <li>You will be automatically connected into Leka!</li>
                </ol>
                <p class="help-note">
                  <strong>Note :</strong> We don't store your private key. 
                  Authentication is performed with your wallet.
                </p>
              </div>
            </details>
          </div>
         </div>
        </main>
    </div>
</template>

<style scoped>

.home-container {
  max-width: 800px;
  margin: 0 auto;
  padding: 2rem;
  text-align: center;
  min-height: 100vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
}

/* Styles héroïques conservés */
.hero-section {
  margin-bottom: 3rem;
}

.app-title {
  font-size: 3rem;
  color: #2c3e50;
  margin-bottom: 1rem;
  font-weight: 700;
  letter-spacing: 0.05em;
}

.app-description {
  font-size: 1.2rem;
  color: #666;
  line-height: 1.6;
  max-width: 600px;
  margin: 0 auto;
  letter-spacing: 0.025em;
}

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

/* Section d'authentification améliorée */
.auth-section {
  margin-bottom: 4rem;
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.login-container {
  background: #f8f9fa;
  padding: 2.5rem;
  border-radius: 16px;
  border: 1px solid #e9ecef;
  max-width: 500px;
  width: 100%;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.08);
}

/* Avertissement MetaMask */
.metamask-warning {
  text-align: center;
  padding: 2rem;
  background: #fff3cd;
  border: 2px solid #ffeaa7;
  border-radius: 12px;
  margin-bottom: 2rem;
}

.warning-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
}

.metamask-warning h3 {
  color: #856404;
  margin-bottom: 1rem;
  font-size: 1.4rem;
}

.metamask-warning p {
  color: #856404;
  margin-bottom: 1.5rem;
  line-height: 1.5;
}

.install-metamask-btn {
  background: #f6851b;
  color: white;
  border: none;
  padding: 1rem 2rem;
  font-size: 1.1rem;
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.3s ease;
  font-weight: 600;
}

.install-metamask-btn:hover {
  background: #e2761b;
  transform: translateY(-2px);
}

/* Interface d'authentification */
.authentication-interface {
  width: 100%;
}

.login-prompt-container {
  margin-bottom: 2rem;
}

.login-prompt, .error-prompt {
  font-size: 1.1rem;
  margin-bottom: 0;
  line-height: 1.5;
}

.login-prompt {
  color: #495057;
}

.error-prompt {
  color: #dc3545;
  font-weight: 500;
}

/* Bouton de connexion Ethereum amélioré */
.ethereum-login-btn {
  background: #ff6b35;
  color: white;
  border: none;
  padding: 1.2rem 2.5rem;
  font-size: 1.1rem;
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s ease;
  font-weight: 600;
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 0.75rem;
  width: 100%;
  position: relative;
  overflow: hidden;
}

.ethereum-login-btn:hover:not(:disabled) {
  background: #e55a2b;
  transform: translateY(-2px);
  box-shadow: 0 8px 25px rgba(255, 107, 53, 0.3);
}

.ethereum-login-btn:disabled {
  cursor: not-allowed;
  transform: none;
}

.ethereum-login-btn.loading {
  background: #6c757d;
}

.ethereum-login-btn.error {
  background: #dc3545;
}

.ethereum-login-btn.error:hover:not(:disabled) {
  background: #c82333;
}

.ethereum-login-btn.success {
  background: #28a745;
}

/* Icônes et animations du bouton */
.btn-icon {
  font-size: 1.2rem;
  display: flex;
  align-items: center;
}

.spinner {
  animation: spin 1s linear infinite;
  display: inline-block;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.btn-text {
  font-weight: 600;
}

/* Conteneur de statut */
.status-container {
  margin-top: 1.5rem;
}

.status-message {
  padding: 1rem;
  border-radius: 8px;
  font-size: 0.95rem;
  font-weight: 500;
  margin-bottom: 1rem;
}

.status-loading {
  background: #cce5ff;
  color: #0056b3;
  border: 1px solid #99d6ff;
}

.status-error {
  background: #f8d7da;
  color: #721c24;
  border: 1px solid #f5c6cb;
}

.status-success {
  background: #d1eddb;
  color: #155724;
  border: 1px solid #c3e6cb;
}

/* Indicateur de progression */
.progress-indicator {
  margin-top: 1rem;
}

.progress-bar {
  background: #e9ecef;
  height: 4px;
  border-radius: 2px;
  overflow: hidden;
  margin-bottom: 0.75rem;
}

.progress-fill {
  height: 100%;
  background: #ff6b35;
  border-radius: 2px;
  transition: width 0.5s ease;
}

.progress-fill.step-connecting { width: 25%; }
.progress-fill.step-challenging { width: 50%; }
.progress-fill.step-signing { width: 75%; }
.progress-fill.step-authenticating { width: 100%; }

.progress-steps {
  display: flex;
  justify-content: space-between;
  font-size: 0.8rem;
  color: #6c757d;
}

.progress-steps span {
  transition: color 0.3s ease;
}

.progress-steps span.active {
  color: #ff6b35;
  font-weight: 600;
}

/* Section d'aide */
.help-section {
  margin-top: 2rem;
  text-align: left;
}

.help-details {
  background: white;
  border: 1px solid #dee2e6;
  border-radius: 8px;
  padding: 1rem;
}

.help-details summary {
  cursor: pointer;
  font-weight: 600;
  color: #495057;
  margin-bottom: 1rem;
}

.help-details[open] summary {
  margin-bottom: 1rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid #dee2e6;
}

.help-content h4 {
  color: #495057;
  margin-bottom: 1rem;
  font-size: 1.1rem;
}

.help-content ol {
  margin-bottom: 1rem;
  padding-left: 1.5rem;
}

.help-content li {
  margin-bottom: 0.5rem;
  line-height: 1.4;
}

.help-note {
  background: #e7f3ff;
  padding: 0.75rem;
  border-radius: 6px;
  border-left: 4px solid #0066cc;
  font-size: 0.9rem;
  line-height: 1.4;
}

/* Section des fonctionnalités conservée */
.features-preview {
  margin-top: 3rem;
}

.features-preview h2 {
  color: #2c3e50;
  margin-bottom: 2rem;
  font-size: 2rem;
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
  border-radius: 12px;
  border: 1px solid #e9ecef;
  transition: transform 0.2s ease;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.05);
}

.feature-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.feature-card h3 {
  color: #2c3e50;
  margin-bottom: 0.75rem;
  font-size: 1.2rem;
}

.feature-card p {
  color: #666;
  font-size: 0.95rem;
  line-height: 1.5;
}

/* Responsive design */
@media (max-width: 768px) {
  .home-container {
    padding: 1rem;
  }
  
  .app-title {
    font-size: 2.5rem;
  }
  
  .login-container {
    padding: 2rem;
  }
  
  .features-grid {
    grid-template-columns: 1fr;
  }
  
  .progress-steps {
    font-size: 0.7rem;
  }
}

/* Animations d'entrée */
@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.authentication-interface {
  animation: fadeInUp 0.5s ease-out;
}
</style>
import { defineStore } from 'pinia'
import { ref, computed, readonly } from 'vue'
import { authService, type User } from '../services/authService'
// import { isEtherAvailable } from '../types/ethereum'

export type AuthState = 
  | 'idle'           // État initial, pas d'action en cours
  | 'connecting'     // Connexion au portefeuille en cours
  | 'challenging'    // Demande de challenge en cours
  | 'signing'        // Signature du message en cours
  | 'authenticating' // Vérification finale en cours
  | 'authenticated'  // Utilisateur connecté avec succès
  | 'error'          // Erreur survenue

export const useAuthStore = defineStore('auth', () => {
    const user = ref<User | null>(null)
    const authState = ref<AuthState>('idle')
    const errorMessage = ref<string>('')
    const isLoading = ref(false)

    const isAuthenticated = computed(() => 
        user.value !== null && authState.value === 'authenticated'
    )

    const isConnecting = computed(() =>
        authState.value === 'connecting' || authState.value === 'challenging'
    )

    const hasError = computed(() =>
        authState.value === 'error'
    )

    const userEthereumAddress = computed(() =>
        user.value ? user.value.ethereum_address : null
    )

    const isAdmin = computed(() =>
        user.value ? user.value.is_admin : false
    )

    const isVerified = computed(() =>
        user.value?.is_verified || false
    )

    async function authenticateWithEther(): Promise<void> {
        try {
            // Reset de l'état précédent
            clearError()
            authState.value = 'connecting'
            isLoading.value = true
      
            console.log('🚀 Début du processus d\'authentification Web3')
      
            // Le service gère tout le flux complexe
            const authenticatedUser = await authService.authenticateWithEther()
      
            // Mise à jour de l'état de succès
            user.value = authenticatedUser
            authState.value = 'authenticated'
            
            console.log('✅ Authentification réussie pour:', authenticatedUser.ethereum_address)
      
          } catch (error: any) {
            // Gestion centralisée des erreurs
            console.error('❌ Erreur d\'authentification:', error.message)
            
            authState.value = 'error'
            errorMessage.value = error.message || 'Une erreur inattendue s\'est produite'
            user.value = null
      
          } finally {
            isLoading.value = false
          }
    }
    
    function initializeAuth(): void {
        console.log('🔄 Initialisation de l\'état d\'authentification...')

        const existingUser = authService.getCurrentUser()
        
        if (existingUser && authService.isAuthenticated()) {
            user.value = existingUser
            authState.value = 'authenticated'
            console.log('🔗 Session utilisateur restaurée:', existingUser.ethereum_address)
        } else {
            console.log('👤 Aucune session active trouvée')
            authState.value = 'idle'
        }
    }

    function logout(): void {
        console.log('👋 Déconnexion de l\'utilisateur')
        
        authService.logout()
        
        user.value = null
        authState.value = 'idle'
        errorMessage.value = ''
        isLoading.value = false
        
        console.log('🧹 État d\'authentification nettoyé')
      }
    
    function clearError(): void {
        errorMessage.value = ''
        if (authState.value === 'error') {
          authState.value = 'idle'
        }
    }

    function resetAuthState(): void {
        console.log('🔄 Remise à zéro complète de l\'état d\'authentification')
        
        // Remise à zéro de tous les états réactifs à leurs valeurs initiales
        user.value = null
        authState.value = 'idle'
        errorMessage.value = ''
        isLoading.value = false
        
        console.log('✨ État d\'authentification réinitialisé à l\'état initial')
    }
    
    function updateUser(updatedUser: User): void {
        if (user.value && user.value.id === updatedUser.id) {
          user.value = updatedUser
          console.log('👤 Informations utilisateur mises à jour')
        }
    }
    
    function checkMetaMaskAvailability(): boolean {
        return isEtherAvailable()
    }
    
    function getErrorDisplayMessage(): string {
        if (!errorMessage.value) return ''
    
        // Mapping des erreurs courantes vers des messages utilisateur
        const errorMappings: Record<string, string> = {
          'MetaMask n\'est pas installé': 'Veuillez installer MetaMask pour vous connecter',
          'Connexion refusée par l\'utilisateur': 'Connexion annulée. Essayez à nouveau.',
          'Signature refusée par l\'utilisateur': 'Signature requise pour la connexion',
          'Invalid signature': 'Signature invalide. Veuillez réessayer.',
          'No active challenge found': 'Session expirée. Veuillez recommencer.',
        }
    
        return errorMappings[errorMessage.value] || 'Une erreur s\'est produite. Veuillez réessayer.'
    }
    
    function trackAuthenticationError(error: string): void {
        // En développement, simple log console
        if (import.meta.env.DEV) {
          console.warn('📊 Erreur d\'authentification trackée:', error)
        }
    }
    
      return {
        // État réactif
        user: readonly(user),
        authState: readonly(authState),
        errorMessage: readonly(errorMessage),
        isLoading: readonly(isLoading),
    
        // Propriétés calculées
        isAuthenticated,
        isConnecting,
        hasError,
        userEthereumAddress,
        isAdmin,
        isVerified,
    
        // Actions
        authenticateWithEther,
        initializeAuth,
        logout,
        clearError,
        resetAuthState,
        updateUser,
        checkMetaMaskAvailability,
        getErrorDisplayMessage,
      }
    })
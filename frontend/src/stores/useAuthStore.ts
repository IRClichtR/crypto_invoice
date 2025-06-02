import { defineStore } from 'pinia'
import { ref, computed, readonly } from 'vue'
import { authService, type User } from '../services/authService'
import { isEtherAvailable } from '../types/ethereum'

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
      
            console.log('🚀 Starting Web3 authentication')
      
            // Le service gère tout le flux complexe
            const authenticatedUser = await authService.authenticateWithEther()
      
            // Mise à jour de l'état de succès
            user.value = authenticatedUser
            authState.value = 'authenticated'
            
            console.log('✅ Authentication successfull for:', authenticatedUser.ethereum_address)
      
          } catch (error: any) {
            // Gestion centralisée des erreurs
            console.error('❌ Authentication error:', error.message)
            
            authState.value = 'error'
            errorMessage.value = error.message || 'Unexpected error occurred'
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
            console.log('🔗 Restaured user session:', existingUser.ethereum_address)
        } else {
            console.log('👤 No active session found')
            authState.value = 'idle'
        }
    }

    function logout(): void {
        console.log('👋 User disconnection')
        
        authService.logout()
        
        user.value = null
        authState.value = 'idle'
        errorMessage.value = ''
        isLoading.value = false
        
        console.log('🧹 Auth State cleaned')
      }
    
    function clearError(): void {
        errorMessage.value = ''
        if (authState.value === 'error') {
          authState.value = 'idle'
        }
    }

    function resetAuthState(): void {
        console.log('🔄 Auth state flushed')
        
        // Remise à zéro de tous les états réactifs à leurs valeurs initiales
        user.value = null
        authState.value = 'idle'
        errorMessage.value = ''
        isLoading.value = false
        
        console.log('✨ Auth state reinitialized')
    }
    
    function updateUser(updatedUser: User): void {
        if (user.value && user.value.id === updatedUser.id) {
          user.value = updatedUser
          console.log('👤 User info updated')
        }
    }
    
    function checkMetaMaskAvailability(): boolean {
        return isEtherAvailable()
    }
    
    function getErrorDisplayMessage(): string {
        if (!errorMessage.value) return ''
    
        // Mapping des erreurs courantes vers des messages utilisateur
        const errorMappings: Record<string, string> = {
          'MetaMask is not installed': 'Please connect to MetaMask to continue.',
          'Connection refused by the User': 'Please restart connexion.',
          'Refused signature': 'Signature required. Please try again.',
          'Invalid signature': 'Invalid Signature. Please try again.',
          'No active challenge found': 'Session expired. Please try again.',
        }
    
        return errorMappings[errorMessage.value] || 'An error has occurred. Please try again.'
    }
    
    function trackAuthenticationError(error: string): void {
        // En développement, simple log console
        if (import.meta.env.DEV) {
          console.warn('📊 Tracked connexion error:', error)
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
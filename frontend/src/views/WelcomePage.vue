<script setup lang="ts">
/**
 * WelcomePage.vue - Dashboard for authenticated users
 * 
 * This page welcomes users after successful authentication and displays
 * their Ethereum information while providing entry points to explore
 * Leka's core functionality. It serves as the main hub once users are logged in.
 */

import { computed, onMounted, ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/useAuthStore'

// Hooks and stores
const authStore = useAuthStore()
const router = useRouter()

// Local state for animations and interactions
const showWelcomeAnimation = ref(true)
const showUserDetails = ref(false)
const showCopyFeedback = ref(false)
const isLoading = ref(false)

/**
 * Computed properties for displaying user data
 * 
 * These reactive properties automatically update when the authentication
 * state changes, ensuring the interface always reflects current user information.
 */
const user = computed(() => authStore.user)
const userAddress = computed(() => authStore.userEthereumAddress)
const isVerified = computed(() => authStore.isVerified)
const isAdmin = computed(() => authStore.isAdmin)

/**
 * Ethereum address formatting for display
 * 
 * Transforms long addresses like "0x1234...abcd" into a readable format
 * that saves screen space while remaining recognizable to users.
 */
const formattedAddress = computed(() => {
  if (!userAddress.value) return ''
  
  const address = userAddress.value
  return `${address.slice(0, 6)}...${address.slice(-4)}`
})

/**
 * Full address for clipboard operations
 */
const fullAddress = computed(() => userAddress.value || '')

/**
 * User status badge styling
 * 
 * Determines the visual styling for user status badges based on their
 * verification level and administrative privileges.
 */
const statusBadgeClass = computed(() => {
  if (isAdmin.value) return 'badge-admin'
  if (isVerified.value) return 'badge-verified'
  return 'badge-standard'
})

const statusText = computed(() => {
  if (isAdmin.value) return 'Administrator'
  if (isVerified.value) return 'Verified'
  return 'Standard'
})

/**
 * Quick actions available to the user
 * 
 * This computed property provides context-aware actions based on user status
 * and current application state, making the interface more intelligent.
 */
const quickActions = computed(() => [
  {
    title: 'Create Invoice',
    description: 'Generate a new invoice for your clients',
    icon: '📄',
    action: () => navigateToInvoices(),
    primary: true
  },
  {
    title: 'View Dashboard',
    description: 'See your transaction history and analytics',
    icon: '📊',
    action: () => navigateToDashboard(),
    primary: false
  },
  {
    title: 'Account Settings',
    description: 'Manage your profile and preferences',
    icon: '⚙️',
    action: () => navigateToSettings(),
    primary: false
  }
])

/**
 * User action handlers
 */

/**
 * Copies the Ethereum address to clipboard
 * 
 * This function provides user-friendly clipboard functionality with visual
 * feedback. It includes fallback support for older browsers that don't
 * support the modern Clipboard API.
 */
async function copyAddressToClipboard(): Promise<void> {
  try {
    await navigator.clipboard.writeText(fullAddress.value)
    
    // Visual feedback for successful copy
    showCopyFeedback.value = true
    setTimeout(() => {
      showCopyFeedback.value = false
    }, 2000)
    
    console.log('✅ Address copied to clipboard successfully')
  } catch (error) {
    console.error('Error copying to clipboard:', error)
    fallbackCopyToClipboard(fullAddress.value)
  }
}

/**
 * Fallback clipboard method for older browsers
 * 
 * This method provides clipboard functionality for browsers that don't
 * support the modern navigator.clipboard API, ensuring broad compatibility.
 */
function fallbackCopyToClipboard(text: string): void {
  const textArea = document.createElement('textarea')
  textArea.value = text
  textArea.style.position = 'fixed'
  textArea.style.opacity = '0'
  
  document.body.appendChild(textArea)
  textArea.select()
  
  try {
    document.execCommand('copy')
    showCopyFeedback.value = true
    setTimeout(() => {
      showCopyFeedback.value = false
    }, 2000)
  } catch (err) {
    console.error('Fallback copy failed:', err)
  }
  
  document.body.removeChild(textArea)
}

/**
 * Navigation handlers for quick actions
 * 
 * These functions handle navigation to different sections of the application.
 * They include loading states and error handling for smooth user experience.
 */
async function navigateToInvoices(): Promise<void> {
  isLoading.value = true
  try {
    await router.push('/invoices')
  } catch (error) {
    console.error('Navigation error:', error)
  } finally {
    isLoading.value = false
  }
}

async function navigateToDashboard(): Promise<void> {
  isLoading.value = true
  try {
    await router.push('/dashboard')
  } catch (error) {
    console.error('Navigation error:', error)
  } finally {
    isLoading.value = false
  }
}

async function navigateToSettings(): Promise<void> {
  isLoading.value = true
  try {
    await router.push('/settings')
  } catch (error) {
    console.error('Navigation error:', error)
  } finally {
    isLoading.value = false
  }
}

/**
 * Logout handler with confirmation
 * 
 * Provides a secure logout process with user confirmation to prevent
 * accidental disconnections, especially important in Web3 applications.
 */
function handleLogout(): void {
  const confirmed = window.confirm('Are you sure you want to disconnect your wallet?')
  
  if (confirmed) {
    console.log('👋 User initiated logout')
    authStore.logout()
    router.push('/')
  }
}

/**
 * Lifecycle hooks
 */

/**
 * Component initialization
 * 
 * Sets up the welcome page experience with staggered animations and
 * authentication verification. This creates a polished onboarding flow.
 */
onMounted(() => {
  console.log('🎉 Welcome page mounted for user:', userAddress.value)
  
  // Verify user is actually authenticated
  if (!authStore.isAuthenticated) {
    console.warn('⚠️ Unauthenticated user accessing welcome page, redirecting...')
    router.push('/')
    return
  }
  
  // Staggered animation reveal for better UX
  setTimeout(() => {
    showWelcomeAnimation.value = false
    showUserDetails.value = true
  }, 800)
  
  // Initialize any user-specific data loading here
  // For example: loadUserInvoices(), loadRecentTransactions(), etc.
})
</script>

<template>
  <div class="welcome-container">
    
    <!-- Header with user greeting and logout -->
    <header class="welcome-header">
      <div class="header-content">
        <div class="greeting-section">
          <h1 class="welcome-title">
            <span class="wave-emoji">👋</span>
            Welcome to Leka!
          </h1>
          <p class="welcome-subtitle">
            Your decentralized invoice management platform is ready to use.
          </p>
        </div>
        
        <button 
          class="logout-btn"
          @click="handleLogout"
          title="Disconnect wallet"
        >
          <span class="logout-icon">🚪</span>
          Logout
        </button>
      </div>
    </header>

    <!-- User information card -->
    <section class="user-info-section">
      <div class="user-card" :class="{ 'show': showUserDetails }">
        <div class="user-card-header">
          <div class="user-avatar">
            <div class="avatar-circle">
              <span class="avatar-icon">🦊</span>
            </div>
          </div>
          
          <div class="user-details">
            <h2 class="user-title">Your Ethereum Account</h2>
            
            <!-- Ethereum address display with copy functionality -->
            <div class="address-container">
              <code class="user-address">{{ formattedAddress }}</code>
              <button 
                class="copy-btn"
                @click="copyAddressToClipboard"
                :class="{ 'copied': showCopyFeedback }"
                title="Copy full address"
              >
                <span v-if="showCopyFeedback" class="copy-success">✅</span>
                <span v-else class="copy-icon">📋</span>
              </button>
            </div>
            
            <!-- Copy feedback message -->
            <div v-if="showCopyFeedback" class="copy-feedback">
              Address copied to clipboard!
            </div>
          </div>

          <!-- User status badge -->
          <div class="user-status">
            <span class="status-badge" :class="statusBadgeClass">
              {{ statusText }}
            </span>
          </div>
        </div>

        <!-- User account information -->
        <div class="user-card-body">
          <div class="account-stats">
            <div class="stat-item">
              <span class="stat-label">Account ID</span>
              <span class="stat-value">{{ user?.id.slice(0, 8) }}...</span>
            </div>
            
            <div class="stat-item">
              <span class="stat-label">Verification Status</span>
              <span class="stat-value" :class="{ 'verified': isVerified }">
                {{ isVerified ? 'Verified ✅' : 'Unverified ⏳' }}
              </span>
            </div>
            
            <div class="stat-item">
              <span class="stat-label">Account Type</span>
              <span class="stat-value">{{ statusText }}</span>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Quick actions section -->
    <section class="quick-actions-section">
      <h2 class="section-title">Quick Actions</h2>
      <p class="section-subtitle">
        Get started with these common tasks or explore the full dashboard.
      </p>
      
      <div class="actions-grid">
        <div 
          v-for="action in quickActions" 
          :key="action.title"
          class="action-card"
          :class="{ 'primary': action.primary }"
          @click="action.action"
        >
          <div class="action-icon">{{ action.icon }}</div>
          <h3 class="action-title">{{ action.title }}</h3>
          <p class="action-description">{{ action.description }}</p>
          <div class="action-arrow">→</div>
        </div>
      </div>
    </section>

    <!-- Getting started guide -->
    <section class="getting-started-section">
      <div class="guide-container">
        <h2 class="guide-title">Getting Started with Leka</h2>
        <div class="guide-steps">
          <div class="step-item">
            <div class="step-number">1</div>
            <div class="step-content">
              <h4>Create Your First Invoice</h4>
              <p>Generate professional invoices that your clients can pay directly with cryptocurrency.</p>
            </div>
          </div>
          
          <div class="step-item">
            <div class="step-number">2</div>
            <div class="step-content">
              <h4>Share with Clients</h4>
              <p>Send invoice links to your clients. They can pay directly without needing a Leka account.</p>
            </div>
          </div>
          
          <div class="step-item">
            <div class="step-number">3</div>
            <div class="step-content">
              <h4>Receive Payments</h4>
              <p>Get paid instantly to your Ethereum wallet with full transparency and security.</p>
            </div>
          </div>
        </div>
      </div>
    </section>

    <!-- Footer with helpful links -->
    <footer class="welcome-footer">
      <div class="footer-content">
        <p class="footer-text">
          Need help? Check out our 
          <a href="#" class="footer-link">documentation</a> 
          or 
          <a href="#" class="footer-link">contact support</a>.
        </p>
      </div>
    </footer>

  </div>
</template>

<style scoped>
/* Welcome page styling with modern, clean design */

.welcome-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #f8f9fa 0%, #e9ecef 100%);
  padding: 2rem;
}

/* Header styling */
.welcome-header {
  margin-bottom: 3rem;
}

.header-content {
  max-width: 1200px;
  margin: 0 auto;
  display: flex;
  justify-content: space-between;
  align-items: center;
  flex-wrap: wrap;
  gap: 2rem;
}

.greeting-section {
  flex: 1;
  min-width: 300px;
}

.welcome-title {
  font-size: 2.5rem;
  font-weight: 700;
  color: #2c3e50;
  margin-bottom: 0.5rem;
  display: flex;
  align-items: center;
  gap: 0.75rem;
}

.wave-emoji {
  font-size: 2rem;
  animation: wave 2s ease-in-out infinite;
}

@keyframes wave {
  0%, 50%, 100% { transform: rotate(0deg); }
  25% { transform: rotate(20deg); }
  75% { transform: rotate(-20deg); }
}

.welcome-subtitle {
  font-size: 1.2rem;
  color: #6c757d;
  line-height: 1.5;
}

.logout-btn {
  background: #dc3545;
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 8px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.logout-btn:hover {
  background: #c82333;
  transform: translateY(-2px);
}

/* User information card */
.user-info-section {
  max-width: 1200px;
  margin: 0 auto 3rem;
}

.user-card {
  background: white;
  border-radius: 16px;
  padding: 2rem;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  border: 1px solid #e9ecef;
  opacity: 0;
  transform: translateY(20px);
  transition: all 0.5s ease;
}

.user-card.show {
  opacity: 1;
  transform: translateY(0);
}

.user-card-header {
  display: flex;
  align-items: flex-start;
  gap: 1.5rem;
  margin-bottom: 2rem;
  flex-wrap: wrap;
}

.user-avatar {
  flex-shrink: 0;
}

.avatar-circle {
  width: 80px;
  height: 80px;
  background: linear-gradient(135deg, #ff6b35, #e55a2b);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 2rem;
}

.user-details {
  flex: 1;
  min-width: 250px;
}

.user-title {
  font-size: 1.5rem;
  font-weight: 600;
  color: #2c3e50;
  margin-bottom: 1rem;
}

.address-container {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 0.5rem;
}

.user-address {
  background: #f8f9fa;
  padding: 0.5rem 1rem;
  border-radius: 6px;
  font-family: 'Courier New', monospace;
  font-size: 1rem;
  border: 1px solid #dee2e6;
  color: #495057;
}

.copy-btn {
  background: #28a745;
  color: white;
  border: none;
  padding: 0.5rem;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 1rem;
}

.copy-btn:hover {
  background: #218838;
  transform: scale(1.05);
}

.copy-btn.copied {
  background: #20c997;
}

.copy-feedback {
  font-size: 0.9rem;
  color: #28a745;
  font-weight: 500;
  animation: fadeIn 0.3s ease;
}

@keyframes fadeIn {
  from { opacity: 0; transform: translateY(-10px); }
  to { opacity: 1; transform: translateY(0); }
}

.user-status {
  flex-shrink: 0;
}

.status-badge {
  padding: 0.5rem 1rem;
  border-radius: 20px;
  font-size: 0.9rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.badge-admin {
  background: #dc3545;
  color: white;
}

.badge-verified {
  background: #28a745;
  color: white;
}

.badge-standard {
  background: #6c757d;
  color: white;
}

.user-card-body {
  border-top: 1px solid #e9ecef;
  padding-top: 2rem;
}

.account-stats {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1.5rem;
}

.stat-item {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.stat-label {
  font-size: 0.9rem;
  color: #6c757d;
  font-weight: 500;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.stat-value {
  font-size: 1.1rem;
  color: #2c3e50;
  font-weight: 600;
}

.stat-value.verified {
  color: #28a745;
}

/* Quick actions section */
.quick-actions-section {
  max-width: 1200px;
  margin: 0 auto 3rem;
}

.section-title {
  font-size: 2rem;
  font-weight: 600;
  color: #2c3e50;
  text-align: center;
  margin-bottom: 0.5rem;
}

.section-subtitle {
  text-align: center;
  color: #6c757d;
  font-size: 1.1rem;
  margin-bottom: 2.5rem;
}

.actions-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1.5rem;
}

.action-card {
  background: white;
  padding: 2rem;
  border-radius: 12px;
  border: 2px solid #e9ecef;
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
}

.action-card:hover {
  transform: translateY(-4px);
  border-color: #ff6b35;
  box-shadow: 0 8px 25px rgba(255, 107, 53, 0.15);
}

.action-card.primary {
  border-color: #ff6b35;
  background: linear-gradient(135deg, #fff 0%, #fff8f6 100%);
}

.action-icon {
  font-size: 3rem;
  margin-bottom: 1rem;
}

.action-title {
  font-size: 1.3rem;
  font-weight: 600;
  color: #2c3e50;
  margin-bottom: 0.75rem;
}

.action-description {
  color: #6c757d;
  line-height: 1.5;
  margin-bottom: 1.5rem;
}

.action-arrow {
  position: absolute;
  bottom: 1rem;
  right: 1rem;
  font-size: 1.5rem;
  color: #ff6b35;
  font-weight: bold;
  transition: transform 0.3s ease;
}

.action-card:hover .action-arrow {
  transform: translateX(5px);
}

/* Getting started guide */
.getting-started-section {
  max-width: 800px;
  margin: 0 auto 3rem;
}

.guide-container {
  background: white;
  padding: 2.5rem;
  border-radius: 16px;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
}

.guide-title {
  font-size: 1.8rem;
  font-weight: 600;
  color: #2c3e50;
  text-align: center;
  margin-bottom: 2rem;
}

.guide-steps {
  display: flex;
  flex-direction: column;
  gap: 2rem;
}

.step-item {
  display: flex;
  align-items: flex-start;
  gap: 1.5rem;
}

.step-number {
  background: #ff6b35;
  color: white;
  width: 40px;
  height: 40px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  flex-shrink: 0;
}

.step-content h4 {
  font-size: 1.2rem;
  font-weight: 600;
  color: #2c3e50;
  margin-bottom: 0.5rem;
}

.step-content p {
  color: #6c757d;
  line-height: 1.5;
}

/* Footer */
.welcome-footer {
  max-width: 1200px;
  margin: 0 auto;
  text-align: center;
  padding-top: 2rem;
  border-top: 1px solid #e9ecef;
}

.footer-text {
  color: #6c757d;
  font-size: 0.95rem;
}

.footer-link {
  color: #ff6b35;
  text-decoration: none;
  font-weight: 500;
}

.footer-link:hover {
  text-decoration: underline;
}

/* Responsive design */
@media (max-width: 768px) {
  .welcome-container {
    padding: 1rem;
  }
  
  .header-content {
    flex-direction: column;
    text-align: center;
  }
  
  .welcome-title {
    font-size: 2rem;
  }
  
  .user-card-header {
    flex-direction: column;
    text-align: center;
  }
  
  .address-container {
    justify-content: center;
  }
  
  .actions-grid {
    grid-template-columns: 1fr;
  }
  
  .step-item {
    flex-direction: column;
    text-align: center;
  }
  
  .account-stats {
    grid-template-columns: 1fr;
  }
}

/* Loading states */
.loading {
  opacity: 0.7;
  pointer-events: none;
}
</style>
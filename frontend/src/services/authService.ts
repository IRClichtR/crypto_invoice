/* Auth service to handle authentication logic 
* Manage auth workflow with Web3 and communication with backend
*/

import { isEtherAvailable, MetaMaskErrorCodes, type EthereumProvider } from '../types/ethereum';


// Types
export interface ChallengeRequest {
    ethereum_address: string;
}

export interface ChallengeResponse {
    challenge_id: string;
    message: string;
    expires_at: string;
}

export interface LoginRequest {
    challenge_id: string;
    ethereum_address: string;
    signature: string;
}

export interface LoginResponse {
    access_token: string;
    refresh_token: string;
    expires_in: number;
    user: {
        id: string;
        ethereum_address: string;
        is_verified: boolean;
        is_admin: boolean;
    };
}

export interface User {
    id: string;
    ethereum_address: string;
    is_verified: boolean;
    is_admin: boolean;
}

// Auth Service class - orchestrator of the authentication 

class AuthService {
    private baseURL: string;
    private currentUser: User | null = null;
    private accessToken: string | null = null;
    private refreshToken: string | null = null;

    constructor() {
        this.baseURL = import.meta.env.VITE_API_BASE_URL || 'http://localhost:8080';

        this.loadTokensFromStorage();
    }

    private checkMetamaskAvailability(): boolean {
        return isEtherAvailable();
    }

    async connectWallet(): Promise<string[]> {
        if (!this.checkMetamaskAvailability()) {
            throw new Error('MetaMask is not available. Please install it to continue.');
        }

        try {
            const accounts = await window.ethereum!.request({
                method: 'eth_requestAccounts'
            });
            if (accounts.length === 0) {
                throw new Error('No accounts found. Please unlock your wallet and try again.');
            }
            return accounts;
        } catch (error: any) {
            if (error.code === MetaMaskErrorCodes.UserRejectedRequest) {
                throw new Error('User rejected the request. Please try again.');
            }
            throw new Error(`Failed to connect wallet: ${error.message}`);
        }
    }

    async requestChallenge(ethereumAddress: string ): Promise<ChallengeResponse> {
        try {
            const response = await fetch(`${this.baseURL}/auth/challenge`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'X-CSRF-Token': this.getCSRFToken(),
                },
                credentials: 'include',
                body: JSON.stringify({
                    ethereum_address: ethereumAddress.toLowerCase()
                })
            });

            if (!response.ok) {
                const errorData = await response.json().catch(() => ({}));
                throw new Error(`Failed to request challenge: ${errorData.message || 'Unknown error'}`);
            }
            return await response.json();
        } catch (error: any) {
            console.error('Error requesting challenge:', error);
            throw new Error(`Failed to request challenge: ${error.message}`);
        }
    }

    async signChallengeWithMetamask(message: string, ethereumAddress: string): Promise<string> {
        if (!this.checkMetamaskAvailability()) {
            throw new Error('MetaMask is not available. Please install it to continue.');
        }

        try {
            const signature = await window.ethereum!.request({
                method: 'personal_sign',
                params: [message, ethereumAddress]
            });

            return signature;
        } catch (error: any) {
            if (error.code === MetaMaskErrorCodes.UserRejectedRequest) {
                throw new Error('User rejected the signature request. Please try again.');
            }
            throw new Error(`Failed to sign challenge: ${error.message}`);
        }
    }

    async login(loginRequest: LoginRequest): Promise<LoginResponse> {
        try {
            const response = await fetch(`${this.baseURL}/auth/login`, {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                    'X-CSRF-Token': this.getCSRFToken(),
                },
                credentials: 'include',
                body: JSON.stringify(loginRequest)
            });

            if (!response.ok) {
                const errorData = await response.json().catch(() => ({}));
                throw new Error(`Login failed: ${errorData.message || 'Unknown error'}`);
            }

            const loginResponse: LoginResponse = await response.json();

            this.saveAuthData(loginResponse);

            return loginResponse;
        } catch (error: any) {
            console.error('Error during login:', error);
            throw new Error(`Login failed: ${error.message}`);
        }   
    }

    private saveAuthData(loginResponse: LoginResponse): void {
        this.currentUser = loginResponse.user;
        this.accessToken = loginResponse.access_token;
        this.refreshToken = loginResponse.refresh_token;
    
        // Stockage persistant pour la reconnexion automatique
        localStorage.setItem('leka_access_token', loginResponse.access_token);
        localStorage.setItem('leka_refresh_token', loginResponse.refresh_token);
        localStorage.setItem('leka_user', JSON.stringify(loginResponse.user));
    }
    
    private loadTokensFromStorage(): void {
        this.accessToken = localStorage.getItem('leka_access_token');
        this.refreshToken = localStorage.getItem('leka_refresh_token');
        
        const userData = localStorage.getItem('leka_user');
        if (userData) {
          try {
            this.currentUser = JSON.parse(userData);
          } catch (error) {
            console.warn('Données utilisateur corrompues dans localStorage');
            this.logout();
          }
        }
    }

    async authenticateWithEther(): Promise<User> {
        try {
            if (!this.checkMetamaskAvailability()) {
                throw new Error('MetaMask is not available. Please install it to continue.');
            }

            const accounts = await this.connectWallet();
            const ethereumAddress = accounts[0].toLowerCase();

            const challengeResponse = await this.requestChallenge(ethereumAddress);
            const signature = await this.signChallengeWithMetamask(challengeResponse.message, ethereumAddress);

            const loginRequest: LoginRequest = {
                challenge_id: challengeResponse.challenge_id,
                ethereum_address: ethereumAddress,
                signature: signature
            };

            const loginResponse = await this.login(loginRequest);

            return loginResponse.user;
        } catch (error: any) {
            console.error('Error during Ether authentication:', error);
            throw new Error(`Authentication failed: ${error.message}`);
        }
    }
    
    private getCSRFToken(): string {
        console.log('Fetching CSRF token from window.BACKEND_CONFIG');
        // TypeScript connaît maintenant la structure exacte de BACKEND_CONFIG
        return window.BACKEND_CONFIG?.csrf_token || '';
    }
    
    getCurrentUser(): User | null {
        return this.currentUser;
    }
    
    getAccessToken(): string | null {
        return this.accessToken;
    }
    
    isAuthenticated(): boolean {
        return this.currentUser !== null && this.accessToken !== null;
    }
    
    logout(): void {
        this.currentUser = null;
        this.accessToken = null;
        this.refreshToken = null;
        
        localStorage.removeItem('leka_access_token');
        localStorage.removeItem('leka_refresh_token');
        localStorage.removeItem('leka_user');
    }
}
    
    // Export d'une instance singleton pour utilisation dans toute l'application
    export const authService = new AuthService();
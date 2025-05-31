/**
 * Module TypeScript pour l'intégration Ethereum/MetaMask
 * 
 * Ce fichier combine les déclarations de types ET le code exécutable
 * pour gérer l'interaction avec les portefeuilles Ethereum. Cette approche
 * permet à la fois une vérification de types stricte et une exécution réelle.
 */

/**
 * Interface pour les événements Ethereum
 * Ces événements sont émis par MetaMask pour notifier les changements
 * d'état du portefeuille (changement de compte, de réseau, etc.)
 */
export interface EthereumEventMap {
    'accountsChanged': (accounts: string[]) => void;
    'chainChanged': (chainId: string) => void;
    'connect': (info: { chainId: string }) => void;
    'disconnect': (error: { code: number; message: string }) => void;
    'message': (message: string) => void;
}

/**
 * Interface principale pour l'objet ethereum injecté par MetaMask
 * 
 * Cette interface définit toutes les méthodes et propriétés disponibles
 * sur window.ethereum. Elle est basée sur les spécifications EIP-1193
 * qui standardisent l'interface des portefeuilles Ethereum.
 */
export interface EthereumProvider {
    isMetaMask?: boolean
    isConnected(): boolean
    readonly chainId: string | null
    readonly networkVersion: string | null
    readonly selectedAddress: string | null
    request<T = any>(request: {
        method: string
        params?: unknown[] | Record<string, unknown>
    }): Promise<T>
    
    on<K extends keyof EthereumEventMap>(
        event: K, 
        listener: EthereumEventMap[K]
    ): void
    removeListener<K extends keyof EthereumEventMap>(
        event: K,
        listener: EthereumEventMap[K]
    ): void
    enable?(): Promise<string[]>
    autoRefreshOnNetworkChange?: boolean
    _metamask?: {
        isUnlocked(): Promise<boolean>
    }
}

/**
 * Types pour les erreurs courantes des portefeuilles Ethereum
 * Basé sur EIP-1193 et les codes d'erreur standard de MetaMask
 */
export interface EthereumRpcError extends Error {
    code: number
    data?: unknown
}

/**
 * Types pour les requêtes RPC courantes
 */
export interface EthereumRequestAccounts {
    method: 'eth_requestAccounts'
    params?: never
}

export interface EthAccounts {
    method: 'eth_accounts'
    params?: never
}

export interface EthChainId {
    method: 'eth_chainId'
    params?: never
}

export interface PersonalSign {
    method: 'personal_sign'
    params: [string, string] // [message, account]
}

export interface EthSendTransaction {
    method: 'eth_sendTransaction'
    params: [{ 
        from: string; 
        to: string; 
        value?: string; 
        gas?: string
        gasPrice?: string
        data?: string 
    }]
}

export interface WalletSwitchEthereumChain {
    method: 'wallet_switchEthereumChain'
    params: [{ chainId: string }]
}

export interface WalletAddEthereumChain {
    method: 'wallet_addEthereumChain'
    params: [{
        chainId: string
        chainName: string
        nativeCurrency: { 
            name: string; 
            symbol: string; 
            decimals: number 
        }
        rpcUrls?: string[]
        blockExplorerUrls?: string[]
    }]
}

/**
 * Union type pour toutes les requêtes RPC possibles
 * Cela nous donne une auto-complétion intelligente dans l'IDE
 */
export type EthereumRpcRequest = 
| EthereumRequestAccounts
| EthAccounts
| EthChainId
| PersonalSign
| EthSendTransaction
| WalletSwitchEthereumChain
| WalletAddEthereumChain;

/**
 * Énumération des codes d'erreur MetaMask les plus courants
 */
export enum MetaMaskErrorCodes {
    UserRejectedRequest = 4001,
    Unauthorized = 4100,
    UnsupportedMethod = 4200,
    Disconnected = 4900,
    ChainDisconnected = 4901,
    UnrecognizedChainId = 4902,
}

/**
 * Types pour les réseaux Ethereum courants
 */
export enum EthereumChainIds {
    Mainnet = '0x1',
    Goerli = '0x5',
    Sepolia = '0xaa36a7',
    Polygon = '0x89',
    BSC = '0x38',
    Localhost = '0x539', // 1337 en décimal
}

/**
 * Interface pour les informations de réseau
 */
export interface NetworkInfo {
    chainId: string
    chainName: string
    nativeCurrency: {
        name: string
        symbol: string
        decimals: number
    }
    rpcUrls: string[]
    blockExplorerUrls: string[]
}

/**
 * Extension de l'interface Window globale
 * 
 * Cette déclaration étend l'interface Window pour inclure l'objet ethereum
 * injecté par MetaMask. TypeScript va maintenant reconnaître window.ethereum
 * comme une propriété typée plutôt que comme 'any'.
 */
declare global {
    interface Window {
        ethereum?: EthereumProvider
        BACKEND_CONFIG?: {
            csrf_token: string
            api_base_url: string
            dev_server_port: number
            assets_path: string
            debug: boolean
        }
        web3?: {
            currentProvider?: EthereumProvider
        }
    }
}

/**
 * FONCTIONS EXÉCUTABLES - La partie cruciale qui manquait
 * 
 * Ces fonctions contiennent la logique réelle qui sera exécutée dans votre application.
 * Elles combinent la vérification de types avec l'implémentation fonctionnelle.
 */

/**
 * Type guard pour vérifier si un portefeuille Ether est disponible
 * 
 * Cette fonction fournit une vérification type-safe de la disponibilité
 * d'un portefeuille Ethereum tout en informant TypeScript du type résultant.
 * Le `: window.ethereum is EthereumProvider` est un "type predicate" qui dit
 * à TypeScript que si cette fonction retourne true, alors window.ethereum
 * est définitivement de type EthereumProvider.
 */
export function isEtherAvailable(): boolean {
    return typeof window !== 'undefined' &&
           typeof window.ethereum === 'object' &&
           window.ethereum !== null &&
           typeof window.ethereum.request === 'function';
}

/**
 * Type guard pour vérifier si c'est MetaMask spécifiquement
 * 
 * Cette fonction permet de distinguer MetaMask d'autres portefeuilles
 * qui pourraient également injecter un objet ethereum dans window.
 */
export function isMetaMask(provider: EthereumProvider): boolean {
    return Boolean(provider.isMetaMask)
}

/**
 * Utilitaire pour obtenir la configuration backend injectée
 * 
 * Cette fonction fournit un accès type-safe à la configuration
 * que votre backend Rust injecte dans window.BACKEND_CONFIG.
 */
export function getBackendConfig() {
    return window.BACKEND_CONFIG
}

/**
 * Fonction utilitaire pour formater les adresses Ethereum
 * 
 * Transforme une adresse complète en format abrégé (ex: 0x1234...abcd)
 * pour l'affichage dans l'interface utilisateur.
 */
export function formatEthereumAddress(address: string, startChars: number = 6, endChars: number = 4): string {
    if (!address || address.length < startChars + endChars) {
        return address
    }
    
    return `${address.slice(0, startChars)}...${address.slice(-endChars)}`
}

/**
 * Fonction utilitaire pour valider les adresses Ethereum
 * 
 * Vérifie qu'une chaîne respecte le format basique d'une adresse Ethereum
 * (commence par 0x, fait 42 caractères, contient uniquement des caractères hexadécimaux)
 */
export function isValidEthereumAddress(address: string): boolean {
    return /^0x[a-fA-F0-9]{40}$/.test(address)
}

/**
 * Fonction utilitaire pour normaliser les adresses Ethereum
 * 
 * Convertit une adresse en minuscules et s'assure qu'elle respecte
 * le format standard. Utile pour les comparaisons d'adresses.
 */
export function normalizeEthereumAddress(address: string): string {
    if (!isValidEthereumAddress(address)) {
        throw new Error(`Invalid Ethereum address: ${address}`)
    }
    
    return address.toLowerCase()
}
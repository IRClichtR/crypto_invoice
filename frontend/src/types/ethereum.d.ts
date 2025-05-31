interface EthereumEventMap {
    'accountsChanged': (accounts: string[]) => void;
    'chainChanged': (chainId: string) => void;
    'connect': (info: { chainId: string }) => void;
    'disconnect': (error: { code: number; message: string }) => void;
    'message': (message: string) => void;
}

interface EthereumProvider {
    isMetamask?: boolean
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

    removeListener<K extends keyof EthereumEventMapo>(
        event: K,
        listener: EthereumEventMap[K]
    ): void

    enable?(): Promise<string[]>

    autoRefreshOnNetworkChange?: boolean
    _metamask?: {
        isUnlocked(): Promise<boolean>
    }
}

// Error types
interface EthereumRcpError extends Error {
    code: number
    data?: unknown
}

interface EthereumRequestAccounts {
    method: 'eth_requestAccounts'
    param?: never
}

interface EthAccounts {
    method: 'eth_accounts'
    param?: never
}

interface EthChainId {
    method: 'eth_chainId'
    param?: never
}

interface PersonalSign {
    method: 'personal_sign'
    params: [string, string] // [message, account]
}

interface EthSendTransaction {
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

interface WalletSwitchEthereumChain {
    method: 'wallet_switchEthereumChain'
    params: [{ chainId: string }]
}

interface WalletAddEthereumChain {
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

type EthereumRcpRequest = 
| EthereumRequestAccounts
| EthAccounts
| EthChainId
| PersonalSign
| EthSendTransaction
| WalletSwitchEthereumChain
| WalletAddEthereumChain;

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

export function isEtherAvailable(): boolean {
    return typeof window !== 'undefined' &&
            typeof window.ethereum === 'object' &&
            typeof window.ethereum.request === 'function';
}

export function isMetamask(provider:EthereumProvider): boolean {
    return Boolean(provider.isMetamask)
}

export enum MetaMaskErrorCodes {
    UserRejectedRequest = 4001,
    Unauthorized = 4100,
    UnsupportedMethod = 4200,
    Diconnected = 4900,
    ChainDisconnected = 4901,
    UnrecognizedChain = 4902,
}
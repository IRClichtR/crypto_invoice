interface BackendConfig {
    csrf_token: string;
    api_url?: string;
    assets_paths?: string;
    debug?: boolean;
}
declare global {
    interface Window {
        BACKEND_CONFIG?: BackendConfig;
    }
}

export {};
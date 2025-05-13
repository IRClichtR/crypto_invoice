interface BackendConfig {
    csrf_token: string;
    api_url: string;
    assets_paths: string;
    debug: boolean;
}

interface Window {
    BACKEND_CONFIG?: BackendConfig;
}

export {};
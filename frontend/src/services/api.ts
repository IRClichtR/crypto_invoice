import axios from 'axios';

const apiClient = axios.create({
    baseURL: window.BACKEND_CONFIG?.api_url || 'http//localhost:8080', // Replace with your API base URL
    headers: {
        'Content-Type': 'application/json',
    },
    withCredentials: true, // Include credentials (cookies) in requests
});

apiClient.interceptors.request.use(config => {
    if (window.BACKEND_CONFIG) {
        config.headers['X-CSRF-Token'] = window.BACKEND_CONFIG.csrf_token;
    }
    return config;
});

export default apiClient;
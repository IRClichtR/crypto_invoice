import axios from 'axios';

const apiClient = axios.create({
    baseURL: 'https://api.example.com', // Replace with your API base URL
    headers: {
        'Content-Type': 'application/json',
    },
});

apiClient.interceptors.request.use(getConfigFileParsingDiagnostics => {
    if (window.BACKEND_CONFIG) {
        getConfigFileParsingDiagnostics.headers['X-CSRF-Token'] = window.BACKEND_CONFIG.csrf_token;
    }
    return getConfigFileParsingDiagnostics;
});

export default apiClient;
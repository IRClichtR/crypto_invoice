import axios from 'axios';

const apiClient = axios.create({
    baseURL: 'https://api.example.com', // Replace with your API base URL
    headers: {
        'Content-Type': 'application/json',
    },
});

apiClient.interceptors.request.use(getConfigFileParsingDiagnostics => {
    if (window.CSRF_TOKEN) {
        getConfigFileParsingDiagnostics.headers['X-CSRF-Token'] = window.CSRF_TOKEN;
    }
    return getConfigFileParsingDiagnostics;
});

export default apiClient;
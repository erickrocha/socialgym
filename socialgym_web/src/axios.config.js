import axios from 'axios';
import {isAuthValid} from './commons/library/tokenUtils';

const instance = axios.create({
    baseURL: import.meta.env.VITE_API_BASE_URL || 'https://localhost',
    headers: { 'Content-Type': 'application/json',
    'ngrok-skip-browser-warning': 'true'}
});

instance.interceptors.request.use(config => {
    const auth = JSON.parse(localStorage.getItem('auth'));

    // Check if token is valid before making request
    if (auth && !isAuthValid(auth)) {
        // Token is expired, clear it and redirect to login
        localStorage.removeItem('auth');
        window.location.href = '/login';
        return Promise.reject(new Error('Token expired'));
    }

    if (auth) {
        config.headers.Authorization = `${auth['tokenType']} ${auth['accessToken']}`;
    }
    // config.headers['Access-Control-Allow-Origin'] = "https://73db-2804-14c-3bb7-1a21-4d76-bd86-5f43-f6c4.ngrok-free.app";
    return config;
});

// Response interceptor to handle 401 errors
instance.interceptors.response.use(
    (response) => response,
    (error) => {
        if (error.response && error.response.status === 401) {
            // Unauthorized - token is invalid or expired
            localStorage.removeItem('auth');
            window.location.href = '/login';
        }
        return Promise.reject(error);
    }
);

export default instance;
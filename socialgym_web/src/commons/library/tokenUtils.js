/**
 * Utility functions for JWT token handling
 */

/**
 * Decodes a JWT token and returns the payload
 * @param {string} token - The JWT token to decode
 * @returns {object|null} - The decoded payload or null if invalid
 */
export const decodeToken = (token) => {
    if (!token) return null;

    try {
        const base64Url = token.split('.')[1];
        const base64 = base64Url.replace(/-/g, '+').replace(/_/g, '/');
        const jsonPayload = decodeURIComponent(
            atob(base64)
                .split('')
                .map((c) => '%' + ('00' + c.charCodeAt(0).toString(16)).slice(-2))
                .join('')
        );
        return JSON.parse(jsonPayload);
    } catch (error) {
        console.error('Error decoding token:', error);
        return null;
    }
};

/**
 * Checks if a JWT token is expired
 * @param {string} token - The JWT token to check
 * @returns {boolean} - True if the token is expired, false otherwise
 */
export const isTokenExpired = (token) => {
    if (!token) return true;

    const decoded = decodeToken(token);
    if (!decoded || !decoded.exp) return true;

    // exp is in seconds, Date.now() returns milliseconds
    const currentTime = Date.now() / 1000;
    return decoded.exp < currentTime;
};

/**
 * Validates if the auth object has a valid, non-expired token
 * @param {object} auth - The auth object containing accessToken
 * @returns {boolean} - True if the token is valid, false otherwise
 */
export const isAuthValid = (auth) => {
    if (!auth || !auth.accessToken) return false;
    return !isTokenExpired(auth.accessToken);
};

/**
 * Gets the remaining time until token expiration in seconds
 * @param {string} token - The JWT token
 * @returns {number} - Seconds until expiration, or 0 if expired/invalid
 */
export const getTokenRemainingTime = (token) => {
    if (!token) return 0;

    const decoded = decodeToken(token);
    if (!decoded || !decoded.exp) return 0;

    const currentTime = Date.now() / 1000;
    const remainingTime = decoded.exp - currentTime;
    return remainingTime > 0 ? Math.floor(remainingTime) : 0;
};


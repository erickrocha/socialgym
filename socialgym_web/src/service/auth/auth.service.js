import axios from "../../axios.config";

const AUTH_STORAGE_KEY = "auth";

/**
 * Re-issues the current JWT with the given business profile active. Team-member
 * management endpoints resolve "the acting business profile" from this claim, not
 * from any route/body parameter, so this must run before those calls.
 */
export const activateBusinessProfileApi = async (businessProfileUuid) => {
    const { data } = await axios.post(`/auth/profile/${businessProfileUuid}/activate`);
    localStorage.setItem(AUTH_STORAGE_KEY, JSON.stringify(data));
    return data;
};

/** Re-issues the JWT back in personal context. */
export const deactivateBusinessProfileApi = async () => {
    const { data } = await axios.post('/auth/profile/deactivate');
    localStorage.setItem(AUTH_STORAGE_KEY, JSON.stringify(data));
    return data;
};

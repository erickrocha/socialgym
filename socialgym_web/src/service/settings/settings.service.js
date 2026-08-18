import axios from "../../axios.config";

export const getMySettingsApi = async () => {
    const { data } = await axios.get('/workout/api/settings/me');
    return data;
};

export const updateMySettingsApi = async (settings) => {
    const { data } = await axios.put('/workout/api/settings/me', settings);
    return data;
};

export const createSettingsApi = async (settings) => {
    const { data } = await axios.post('/workout/api/settings', settings);
    return data;
};

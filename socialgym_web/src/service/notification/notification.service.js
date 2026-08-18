import axios from "../../axios.config";

export const getNotificationsApi = async () => {
    const { data } = await axios.get('/timeline/api/notifications');
    return data;
};

export const markNotificationAsReadApi = async (id) => {
    const { data } = await axios.put(`/timeline/api/notifications/${id}/read`);
    return data;
};

export const markAllNotificationsAsReadApi = async () => {
    const { data } = await axios.put('/timeline/api/notifications/read-all');
    return data;
};

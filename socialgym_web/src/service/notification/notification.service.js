import axios from "../../axios.config";

// The timeline API keeps notifications under their owner: /notifications/{ownerUuid}. It names a
// notification by `uuid` and describes it with `actorName` and `snippet`; the screens read `id`, `title` and
// `created_at`.
const forScreens = (notification) => ({
    ...notification,
    id: notification.id ?? notification.uuid,
    // The bell shows the title alone, so it carries who did it and what.
    title: notification.title ?? [notification.actorName, notification.snippet].filter(Boolean).join(': '),
    message: notification.message,
    created_at: notification.created_at ?? notification.createdAt,
});

export const getNotificationsApi = async (ownerUuid) => {
    const { data } = await axios.get(`/timeline/api/notifications/${ownerUuid}`);
    return Array.isArray(data) ? data.map(forScreens) : [];
};

export const markNotificationAsReadApi = async (ownerUuid, id) => {
    const { data } = await axios.put(`/timeline/api/notifications/${ownerUuid}/read/${id}`);
    return data;
};

// The API marks one notification at a time.
export const markAllNotificationsAsReadApi = async (ownerUuid, ids) => {
    await Promise.all(ids.map((id) => markNotificationAsReadApi(ownerUuid, id)));
};

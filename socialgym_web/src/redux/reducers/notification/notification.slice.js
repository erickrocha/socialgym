import { createSlice } from "@reduxjs/toolkit";
import { fetchNotifications, markAsRead, markAllAsRead } from "./notification.actions";

const initialState = {
    notifications: [],
    unreadCount: 0,
    loading: false,
    error: null,
};

const notificationSlice = createSlice({
    name: 'notification',
    initialState,
    reducers: {},
    extraReducers: (builder) => {
        builder
            .addCase(fetchNotifications.pending, (state) => {
                state.loading = true;
            })
            .addCase(fetchNotifications.fulfilled, (state, { payload }) => {
                state.loading = false;
                state.notifications = Array.isArray(payload) ? payload : [];
                state.unreadCount = state.notifications.filter(n => !n.read).length;
            })
            .addCase(fetchNotifications.rejected, (state, { payload }) => {
                state.loading = false;
                state.error = payload;
            })
            .addCase(markAsRead.fulfilled, (state, { payload }) => {
                state.notifications = state.notifications.map(n =>
                    n.id === payload ? { ...n, read: true } : n
                );
                state.unreadCount = Math.max(0, state.unreadCount - 1);
            })
            .addCase(markAllAsRead.fulfilled, (state) => {
                state.notifications = state.notifications.map(n => ({ ...n, read: true }));
                state.unreadCount = 0;
            });
    }
});

export default notificationSlice.reducer;

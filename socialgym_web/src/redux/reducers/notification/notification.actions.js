import { createAsyncThunk } from "@reduxjs/toolkit";
import { getNotificationsApi, markNotificationAsReadApi, markAllNotificationsAsReadApi } from "../../../service/notification/notification.service";
import { decodeToken } from "../../../commons/library/tokenUtils";

// Whose notifications these are: the person on screen, else the one in the access token.
const ownerUuid = (state) =>
    state.person?.person?.uuid || decodeToken(state.auth?.auth?.accessToken || '')?.person_uuid;

export const fetchNotifications = createAsyncThunk(
    'notification/fetchNotifications',
    async (_, { rejectWithValue, getState }) => {
        try {
            return await getNotificationsApi(ownerUuid(getState()));
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

export const markAsRead = createAsyncThunk(
    'notification/markAsRead',
    async (id, { rejectWithValue, getState }) => {
        try {
            await markNotificationAsReadApi(ownerUuid(getState()), id);
            return id;
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

export const markAllAsRead = createAsyncThunk(
    'notification/markAllAsRead',
    async (_, { rejectWithValue, getState }) => {
        try {
            const state = getState();
            const unread = (state.notification?.notifications || []).filter((n) => !n.read).map((n) => n.id);
            await markAllNotificationsAsReadApi(ownerUuid(state), unread);
            return true;
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

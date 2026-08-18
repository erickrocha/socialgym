import { createAsyncThunk } from "@reduxjs/toolkit";
import { getNotificationsApi, markNotificationAsReadApi, markAllNotificationsAsReadApi } from "../../../service/notification/notification.service";

export const fetchNotifications = createAsyncThunk(
    'notification/fetchNotifications',
    async (_, { rejectWithValue }) => {
        try {
            return await getNotificationsApi();
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

export const markAsRead = createAsyncThunk(
    'notification/markAsRead',
    async (id, { rejectWithValue }) => {
        try {
            await markNotificationAsReadApi(id);
            return id;
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

export const markAllAsRead = createAsyncThunk(
    'notification/markAllAsRead',
    async (_, { rejectWithValue }) => {
        try {
            await markAllNotificationsAsReadApi();
            return true;
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

import { createAsyncThunk } from "@reduxjs/toolkit";
import { getMySettingsApi, updateMySettingsApi, createSettingsApi } from "../../../service/settings/settings.service";

export const fetchMySettings = createAsyncThunk(
    'settings/fetchMySettings',
    async (_, { rejectWithValue }) => {
        try {
            return await getMySettingsApi();
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

export const updateMySettings = createAsyncThunk(
    'settings/updateMySettings',
    async (settingsData, { rejectWithValue }) => {
        try {
            return await updateMySettingsApi(settingsData);
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

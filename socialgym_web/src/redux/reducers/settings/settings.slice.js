import { createSlice } from "@reduxjs/toolkit";
import { fetchMySettings, updateMySettings } from "./settings.actions";

const initialState = {
    settings: {
        language: 'pt-BR',
        theme: 'dark',
        notificationsEnabled: true,
        contextMenuPosition: 'Left',
        homePage: '/home'
    },
    loading: false,
    error: null,
};

const settingsSlice = createSlice({
    name: 'settings',
    initialState,
    reducers: {
        setLocalSettings: (state, { payload }) => {
            state.settings = { ...state.settings, ...payload };
        }
    },
    extraReducers: (builder) => {
        builder
            .addCase(fetchMySettings.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(fetchMySettings.fulfilled, (state, { payload }) => {
                state.loading = false;
                if (payload) {
                    state.settings = { ...state.settings, ...payload };
                }
            })
            .addCase(fetchMySettings.rejected, (state, { payload }) => {
                state.loading = false;
                state.error = payload;
            })
            .addCase(updateMySettings.fulfilled, (state, { payload }) => {
                state.loading = false;
                if (payload) {
                    state.settings = { ...state.settings, ...payload };
                }
            });
    }
});

export const { setLocalSettings } = settingsSlice.actions;
export default settingsSlice.reducer;

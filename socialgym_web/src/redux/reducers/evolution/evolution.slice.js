import { createSlice } from "@reduxjs/toolkit";
import { fetchEvolutionCheckins, createEvolutionCheckin, deleteEvolutionCheckin } from "./evolution.actions";

const initialState = {
    checkins: [],
    loading: false,
    error: null,
};

const evolutionSlice = createSlice({
    name: 'evolution',
    initialState,
    reducers: {},
    extraReducers: (builder) => {
        builder
            .addCase(fetchEvolutionCheckins.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(fetchEvolutionCheckins.fulfilled, (state, { payload }) => {
                state.loading = false;
                state.checkins = Array.isArray(payload) ? payload : [];
            })
            .addCase(fetchEvolutionCheckins.rejected, (state, { payload }) => {
                state.loading = false;
                state.error = payload;
            })
            .addCase(createEvolutionCheckin.fulfilled, (state, { payload }) => {
                state.loading = false;
                state.checkins.unshift(payload);
            })
            .addCase(deleteEvolutionCheckin.fulfilled, (state, { payload }) => {
                state.loading = false;
                state.checkins = state.checkins.filter(c => c.id !== payload && c._id !== payload);
            });
    }
});

export default evolutionSlice.reducer;

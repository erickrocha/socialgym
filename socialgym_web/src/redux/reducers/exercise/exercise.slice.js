import { createSlice } from "@reduxjs/toolkit";
import {
    fetchExercises,
    fetchExerciseById,
    createExercise,
    updateExercise,
    deleteExercise
} from "./exercise.actions";

const initialState = {
    exercises: [],
    selectedExercise: null,
    loading: false,
    error: null,
    totalCount: 0,
};

const exerciseSlice = createSlice({
    name: 'exercise',
    initialState,
    reducers: {
        setSelectedExercise: (state, { payload }) => {
            state.selectedExercise = payload;
        },
        clearSelectedExercise: (state) => {
            state.selectedExercise = null;
        },
        clearExerciseError: (state) => {
            state.error = null;
        }
    },
    extraReducers: (builder) => {
        builder
            .addCase(fetchExercises.pending, (state) => {
                state.loading = true;
                state.error = null;
            })
            .addCase(fetchExercises.fulfilled, (state, { payload }) => {
                state.loading = false;
                state.exercises = Array.isArray(payload) ? payload : (payload.content || []);
                state.totalCount = payload.total_count || state.exercises.length;
            })
            .addCase(fetchExercises.rejected, (state, { payload }) => {
                state.loading = false;
                state.error = payload;
            })

            .addCase(createExercise.fulfilled, (state, { payload }) => {
                state.exercises.unshift(payload);
                state.loading = false;
            })
            .addCase(updateExercise.fulfilled, (state, { payload }) => {
                state.exercises = state.exercises.map(ex => ex.id === payload.id ? payload : ex);
                if (state.selectedExercise?.id === payload.id) {
                    state.selectedExercise = payload;
                }
                state.loading = false;
            })
            .addCase(deleteExercise.fulfilled, (state, { payload }) => {
                state.exercises = state.exercises.filter(ex => ex.id !== payload);
                if (state.selectedExercise?.id === payload) {
                    state.selectedExercise = null;
                }
                state.loading = false;
            });
    }
});

export const { setSelectedExercise, clearSelectedExercise, clearExerciseError } = exerciseSlice.actions;
export default exerciseSlice.reducer;

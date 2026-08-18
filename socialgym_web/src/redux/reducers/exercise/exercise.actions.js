import { createAsyncThunk } from "@reduxjs/toolkit";
import {
    getExercisesApi,
    getExerciseByIdApi,
    createExerciseApi,
    updateExerciseApi,
    deleteExerciseApi
} from "../../../service/exercise/exercise.service";

export const fetchExercises = createAsyncThunk(
    'exercise/fetchExercises',
    async (params, { rejectWithValue }) => {
        try {
            return await getExercisesApi(params);
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

export const fetchExerciseById = createAsyncThunk(
    'exercise/fetchExerciseById',
    async (id, { rejectWithValue }) => {
        try {
            return await getExerciseByIdApi(id);
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

export const createExercise = createAsyncThunk(
    'exercise/createExercise',
    async (exerciseData, { rejectWithValue }) => {
        try {
            return await createExerciseApi(exerciseData);
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

export const updateExercise = createAsyncThunk(
    'exercise/updateExercise',
    async ({ id, data }, { rejectWithValue }) => {
        try {
            return await updateExerciseApi(id, data);
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

export const deleteExercise = createAsyncThunk(
    'exercise/deleteExercise',
    async (id, { rejectWithValue }) => {
        try {
            await deleteExerciseApi(id);
            return id;
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

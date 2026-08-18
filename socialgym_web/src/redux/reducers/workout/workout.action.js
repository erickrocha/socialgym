
import axios from "../../../axios.config";
import {createAsyncThunk} from "@reduxjs/toolkit";

export const getWorkouts = createAsyncThunk(
    'workout/getWorkouts',
    async (personId, {rejectWithValue}) => {
        try {
            const {data} = await axios.get(`/workout/api/workouts/${personId}`);
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
)

export const addWorkout = createAsyncThunk(
    'workout/addWorkout',
    async (workout, {rejectWithValue}) => {
        try {
            const {data} = await axios.post('/workout/api/workouts', workout);
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
)

export const addExercise = createAsyncThunk(
    'workout/addExercise',
    async (exercise, {rejectWithValue}) => {
        try {
            const {data} = await axios.post('/api/exercises', exercise);
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
)

export const addExercises = createAsyncThunk(
    'workout/addExercises',
    async (payload, {rejectWithValue}) => {
        try {
            const {data} = await axios.post(`/api/workouts/${payload.workoutId}/exercises`, payload.exercises);
            return { workoutId: payload.workoutId, exercises: data };
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
)


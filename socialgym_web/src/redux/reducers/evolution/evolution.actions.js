import { createAsyncThunk } from "@reduxjs/toolkit";
import { getEvolutionCheckinsApi, createEvolutionCheckinApi, deleteEvolutionCheckinApi } from "../../../service/evolution/evolution.service";

export const fetchEvolutionCheckins = createAsyncThunk(
    'evolution/fetchEvolutionCheckins',
    async (personId, { rejectWithValue }) => {
        try {
            return await getEvolutionCheckinsApi(personId);
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

export const createEvolutionCheckin = createAsyncThunk(
    'evolution/createEvolutionCheckin',
    async (checkinData, { rejectWithValue }) => {
        try {
            return await createEvolutionCheckinApi(checkinData);
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

export const deleteEvolutionCheckin = createAsyncThunk(
    'evolution/deleteEvolutionCheckin',
    async (id, { rejectWithValue }) => {
        try {
            await deleteEvolutionCheckinApi(id);
            return id;
        } catch (error) {
            return rejectWithValue(error.response?.data || error.message);
        }
    }
);

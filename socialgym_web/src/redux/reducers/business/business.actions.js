import axios from "../../../axios.config";
import {createAsyncThunk} from "@reduxjs/toolkit";

export const getBusinessProfiles = createAsyncThunk(
    'business/getBusinessProfiles',
    async (_, {rejectWithValue}) => {
        try {
            const {data} = await axios.get('/workout/api/business-profiles');
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data || 'Error fetching business profiles');
        }
    }
);

export const getBusinessProfileById = createAsyncThunk(
    'business/getBusinessProfileById',
    async (id, {rejectWithValue}) => {
        try {
            const {data} = await axios.get(`/workout/api/business-profiles/${id}`);
            return data;
        } catch (error) {
            return rejectWithValue(error.response?.data || 'Error fetching business profile');
        }
    }
);

export const updateBusinessProfile = createAsyncThunk(
    'business/updateBusinessProfile',
    async (profile, {rejectWithValue}) => {
        try {
            const {data} = await axios.put(`/api/business-profiles/${profile.id}`, profile);
            return data;
        } catch (error) {
            return rejectWithValue(error.response?.data || 'Error updating business profile');
        }
    }
);


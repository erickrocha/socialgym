import axios from "../../../axios.config";
import {createAsyncThunk} from "@reduxjs/toolkit";
import {uploadFileToS3} from "../../../service/s3/index.js";

export const getMe = createAsyncThunk(
    'person/getMe',
    async (_, {rejectWithValue}) => {
        try {
            const {data} = await axios.get('/workout/api/people/me');
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
);

export const updatePerson = createAsyncThunk(
    'person/updatePerson',
    async (person, {rejectWithValue}) => {
        try {
            const {data} = await axios.put('/workout/api/people/me', person);
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
)

export const updatePersonInfo = createAsyncThunk(
    'person/updatePersonInfo',
    async (personInfo, {rejectWithValue}) => {
        try {
            const {data} = await axios.put(`/api/person_info/${personInfo.id}`, personInfo);
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
)

export const uploadCoverImage = createAsyncThunk(
    'person/uploadCoverImage',
    async (file, {rejectWithValue}) => {
        try {
            const {data} = await axios.post(`/api/me/upload/cover?format=${file.type}`);
            await uploadFileToS3(file, data.url)
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
)

export const uploadAvatar = createAsyncThunk(
    'person/uploadAvatar',
    async (payload, {rejectWithValue}) => {
        try {
            const {data} = await axios.post(`/api/me/upload/avatar?format=${payload?.format}`);
            await uploadFileToS3(payload?.file, data.url,payload?.format)
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
)
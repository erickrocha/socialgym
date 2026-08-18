import axios from "../../../axios.config";
import {createAsyncThunk} from "@reduxjs/toolkit";

export const authentication = createAsyncThunk(
    "auth/authentication",
    async (payload, {rejectWithValue}) => {
        try {
            const body = new URLSearchParams();
            body.append('email', payload.email);
            body.append('password', payload.password);
            const {data} = await axios.post('/login', body, {
                headers: {
                    'Content-Type': 'application/x-www-form-urlencoded'
                }
            })
            localStorage.setItem("auth", JSON.stringify(data))
            return data
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
);

export const signUp = createAsyncThunk(
    'auth/signUp',
    async (userRegister, {rejectWithValue}) => {
        try {
            const {data} = await axios.post('/signup', userRegister);
            localStorage.setItem("auth", JSON.stringify(data))
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
);

import axios from "../../../axios.config";
import {createAsyncThunk} from "@reduxjs/toolkit";

export const getFriends = createAsyncThunk(
    'friend/page',
    async (_, {rejectWithValue}) => {
        try {
            const {data} = await axios.get(`/workout/api/friends`);
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
)

export const sendFriendRequest = createAsyncThunk(
    'friend/sendFriendRequest',
    async (receiverId, {rejectWithValue}) => {
        try {
            const {data} = await axios.put(`/workout/api/friend/request/${receiverId}`);
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
)

export const acceptFriendRequest = createAsyncThunk(
    'friend/acceptFriendRequest',
    async (receiverId, {rejectWithValue}) => {
        try {
            const {data} = await axios.put(`/api/friend/accept/${receiverId}`);
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
)

export const denyFriendRequest = createAsyncThunk(
    'friend/denyFriendRequest',
    async (receiverId, {rejectWithValue}) => {
        try {
            const {data} = await axios.put(`/api/friend/deny/${receiverId}`);
            return data;
        } catch (error) {
            return rejectWithValue(error.response.data);
        }
    }
)
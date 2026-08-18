import {updateObject} from "../../../commons/library/utility";
import {createSlice} from "@reduxjs/toolkit";

import {acceptFriendRequest, denyFriendRequest, getFriends, sendFriendRequest} from "./friend.action";

const initialState = {
    friends: [],
    suggestions: [],
    receiveRequests: [],
    sentRequests: [],
    loading: false,
    error: null
}

const friendSlice = createSlice({
    name: 'friend',
    initialState,
    reducers: {},
    extraReducers: (builder) => {
        builder
            .addCase(getFriends.pending, (state) => {
                return updateObject(state, {
                    loading: true,
                    error: null
                })
            })
            .addCase(getFriends.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    ...payload
                })
            })
            .addCase(getFriends.rejected, (state, {payload}) => (
                {
                    ...state,
                    loading: false,
                    error: payload
                }
            ))
            .addCase(sendFriendRequest.pending, (state) => {
                return updateObject(state, {
                    loading: true,
                    error: null
                })
            })
            .addCase(sendFriendRequest.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    sentRequests: [...state.sentRequests, payload]
                })
            })
            .addCase(sendFriendRequest.rejected, (state, {payload}) => (
                {
                    ...state,
                    loading: false,
                }
            ))
            .addCase(acceptFriendRequest.pending, (state) => {
                return updateObject(state, {
                    loading: true,
                    error: null
                })
            })
            .addCase(acceptFriendRequest.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    friends: [...state.friends, payload]
                })
            })
            .addCase(acceptFriendRequest.rejected, (state, {payload}) => (
                {
                    ...state,
                    loading: false,
                }
            ))
            .addCase(denyFriendRequest.pending, (state) => {
                return updateObject(state, {
                    loading: true,
                    error: null
                })
            })
            .addCase(denyFriendRequest.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                })
            })
            .addCase(denyFriendRequest.rejected, (state, {payload}) => (
                {
                    ...state,
                    loading: false,
                }
            ))
    }
})

export default friendSlice.reducer;
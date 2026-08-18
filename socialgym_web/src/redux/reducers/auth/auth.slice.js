import {updateObject} from "../../../commons/library/utility";
import {createSlice} from "@reduxjs/toolkit";
import {authentication, signUp} from "./auth.actions.js";

const AUTH = "auth";
const auth = localStorage.getItem(AUTH) ? JSON.parse(localStorage.getItem(AUTH)) : null;

const initialState = {
    loading: false,
    auth,
    user: null,
    error: null,
    success: false
}

const authSlice = createSlice({
    name: 'auth',
    initialState,
    reducers: {
        logout: (state) => {
            localStorage.removeItem(AUTH) // deletes token from storage
            state.loading = false
            state.auth = null
            state.success = false
            state.error = null
        },
        setCredentials: (state, {payload}) => {
            state.auth = payload
        },
    },
    extraReducers: (builder) => {
        builder
            .addCase(authentication.pending, (state) => {
                return updateObject(state, {
                    loading: true,
                    error: null
                })
            })
            .addCase(authentication.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    auth: payload,
                    success: true
                })
            })
            .addCase(authentication.rejected, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    error: payload
                })
            })
            .addCase(signUp.pending, (state) => {
                return updateObject(state, {
                    loading: true,
                    error: null
                })
            })
            .addCase(signUp.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    auth: payload,
                    success: true
                })
            })
            .addCase(signUp.rejected, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    error: payload
                })
            })
    }
})

export const {logout, setCredentials} = authSlice.actions
export default authSlice.reducer;
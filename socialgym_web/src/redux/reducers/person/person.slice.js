import {updateObject} from "../../../commons/library/utility";
import {createSlice} from "@reduxjs/toolkit";
import {getMe, updatePerson, updatePersonInfo,uploadCoverImage} from "./person.actions.js";


const initialState = {
    person: null,
    loading: false,
    error: null
}

const personSlice = createSlice({
    name: 'person',
    initialState,
    reducers: {},
    extraReducers: (builder) => {
        builder
            .addCase(getMe.pending, (state) => {
                return updateObject(state, {
                    loading: true,
                    error: null
                })
            })
            .addCase(getMe.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    person: payload
                })
            })
            .addCase(getMe.rejected, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    error: payload
                })
            })
            .addCase(updatePerson.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    person: payload
                })
            })
            .addCase(updatePerson.rejected, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    error: payload
                })
            })
            .addCase(updatePerson.pending, (state) => {
                return updateObject(state, {
                    loading: true,
                    error: null
                })
            })
            .addCase(updatePersonInfo.pending, (state) => {
                return updateObject(state, {
                    loading: true,
                    error: null
                })
            })
            .addCase(updatePersonInfo.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    person: {
                        ...state.person,
                        personInfo: payload
                    },
                    loading: false
                })
            })
            .addCase(updatePersonInfo.rejected, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    error: payload
                })
            })
    }

});

export default personSlice.reducer;
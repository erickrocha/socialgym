import {updateObject} from "../../../commons/library/utility";
import {createSlice} from "@reduxjs/toolkit";
import {getBusinessProfiles, getBusinessProfileById, updateBusinessProfile} from "./business.actions.js";

const initialState = {
    businessProfiles: [],
    selectedProfile: null,
    loading: false,
    error: null
};

const businessSlice = createSlice({
    name: 'business',
    initialState,
    reducers: {
        clearSelectedProfile: (state) => {
            return updateObject(state, {
                selectedProfile: null
            });
        }
    },
    extraReducers: (builder) => {
        builder
            .addCase(getBusinessProfiles.pending, (state) => {
                return updateObject(state, {
                    loading: true,
                    error: null
                });
            })
            .addCase(getBusinessProfiles.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    businessProfiles: payload
                });
            })
            .addCase(getBusinessProfiles.rejected, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    error: payload
                });
            })
            .addCase(getBusinessProfileById.pending, (state) => {
                return updateObject(state, {
                    loading: true,
                    error: null
                });
            })
            .addCase(getBusinessProfileById.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    selectedProfile: payload
                });
            })
            .addCase(getBusinessProfileById.rejected, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    error: payload
                });
            })
            .addCase(updateBusinessProfile.pending, (state) => {
                return updateObject(state, {
                    loading: true,
                    error: null
                });
            })
            .addCase(updateBusinessProfile.fulfilled, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    selectedProfile: payload,
                    businessProfiles: state.businessProfiles.map(p =>
                        p.id === payload.id ? payload : p
                    )
                });
            })
            .addCase(updateBusinessProfile.rejected, (state, {payload}) => {
                return updateObject(state, {
                    loading: false,
                    error: payload
                });
            });
    }
});

export const {clearSelectedProfile} = businessSlice.actions;
export default businessSlice.reducer;


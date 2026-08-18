// features/upload/uploadSlice.js
import { createSlice } from '@reduxjs/toolkit';

const uploadSlice = createSlice({
    name: 'upload',
    initialState: {
        percentage: 0,
        isUploading: false,
        error: null,
    },
    reducers: {
        startUpload: (state) => {
            state.isUploading = true;
            state.percentage = 0;
            state.error = null;
        },
        setUploadProgress: (state, action) => {
            state.percentage = action.payload;
        },
        finishUpload: (state) => {
            state.isUploading = false;
            state.percentage = 100;
        },
        uploadError: (state, action) => {
            state.isUploading = false;
            state.error = action.payload;
        },
    },
});

export const { startUpload, setUploadProgress, finishUpload, uploadError } = uploadSlice.actions;
export default uploadSlice.reducer;
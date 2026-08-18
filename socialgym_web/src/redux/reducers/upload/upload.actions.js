// features/upload/upload.actions.js
import axios from 'axios';
import { startUpload, setUploadProgress, finishUpload, uploadError } from './upload.slice.js';

export const uploadFileToS3 = (file, presignedUrl) => async (dispatch) => {
    try {
        dispatch(startUpload());

        await axios.put(presignedUrl, file, {
            headers: {
                'Content-Type': file.type,
            },
            onUploadProgress: (progressEvent) => {
                // Cálculo do percentual
                const percentCompleted = Math.round(
                    (progressEvent.loaded * 100) / progressEvent.total
                );
                if (percentCompleted % 5 === 0) {
                    dispatch(setUploadProgress(percentCompleted));
                }
            },
        });

        dispatch(finishUpload());
    } catch (error) {
        dispatch(uploadError(error.message));
    }
};
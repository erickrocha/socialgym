import React from 'react';
import { useDispatch, useSelector } from 'react-redux';
import { uploadFileToS3 } from '../../../redux/reducers/upload/index.js';

const Uploader = ({ presignedUrl }) => {
    const dispatch = useDispatch();
    const { percentage, isUploading } = useSelector((state) => state.upload);

    const handleFileChange = (e) => {
        const file = e.target.files[0];
        if (file && presignedUrl) {
            dispatch(uploadFileToS3(file, presignedUrl));
        }
    };

    return (
        <div>
            <input type="file" onChange={handleFileChange} />

            {isUploading && (
                <div style={{ width: '100%', backgroundColor: '#ddd' }}>
                    <div style={{
                        width: `${percentage}%`,
                        backgroundColor: 'green',
                        color: 'white',
                        textAlign: 'center'
                    }}>
                        {percentage}%
                    </div>
                </div>
            )}
        </div>
    );
};

export default Uploader;
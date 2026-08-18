import axios from 'axios';


const uploadFileToS3 = async (file, url) => {
    const response = await axios.put(url, file, {
        headers: {
            'Content-Type': file.type
        }
    });
    return response.data;
}

export {uploadFileToS3};

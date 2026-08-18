import axios from "../../axios.config";

export const getExercisesApi = async (params = {}) => {
    const { data } = await axios.get('/workout/api/exercise', { params });
    return data;
};

export const getExerciseByIdApi = async (id) => {
    const { data } = await axios.get(`/workout/api/exercise/${id}`);
    return data;
};

export const createExerciseApi = async (exerciseData) => {
    const { data } = await axios.post('/workout/api/exercise', exerciseData);
    return data;
};

export const updateExerciseApi = async (id, exerciseData) => {
    const { data } = await axios.put(`/workout/api/exercise/${id}`, exerciseData);
    return data;
};

export const deleteExerciseApi = async (id) => {
    const { data } = await axios.delete(`/workout/api/exercise/${id}`);
    return data;
};

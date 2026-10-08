import axios from "../../axios.config";

// The exercise routes of the workout API: a paginated query (POST), read by id, create and update
// (the id travels in the body), delete by id.
export const getExercisesApi = async (params = {}) => {
    const { data } = await axios.post('/workout/api/exercises/query', { pageNumber: 1, pageSize: 100, ...params });
    return data;
};

export const getExerciseByIdApi = async (id) => {
    const { data } = await axios.get(`/workout/api/exercises/id/${id}`);
    return data;
};

export const createExerciseApi = async (exerciseData) => {
    const { data } = await axios.post('/workout/api/exercises', exerciseData);
    return data;
};

export const updateExerciseApi = async (id, exerciseData) => {
    const { data } = await axios.put('/workout/api/exercises', { ...exerciseData, id });
    return data;
};

export const deleteExerciseApi = async (id) => {
    const { data } = await axios.delete(`/workout/api/exercises/${id}`);
    return data;
};

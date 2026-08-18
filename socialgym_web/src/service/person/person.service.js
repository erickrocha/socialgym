import axios from "../../axios.config";

export const searchPersonsApi = async (query, limit = 10) => {
    const { data } = await axios.get('/workout/api/people/search', { params: { query, limit } });
    return data;
};

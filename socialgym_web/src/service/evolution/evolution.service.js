import axios from "../../axios.config";

export const getEvolutionCheckinsApi = async () => {
    // The API lists the caller's own check-ins; there is no per-person route.
    const { data } = await axios.get('/timeline/api/evolution-checkin');
    return data;
};

export const createEvolutionCheckinApi = async (checkinData) => {
    const { data } = await axios.post('/timeline/api/evolution-checkin', checkinData);
    return data;
};

export const deleteEvolutionCheckinApi = async (id) => {
    const { data } = await axios.delete(`/timeline/api/evolution-checkins/${id}`);
    return data;
};

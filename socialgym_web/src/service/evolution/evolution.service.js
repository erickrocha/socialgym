import axios from "../../axios.config";

export const getEvolutionCheckinsApi = async (personId) => {
    const url = personId ? `/timeline/api/evolution-checkins/person/${personId}` : '/timeline/api/evolution-checkins';
    const { data } = await axios.get(url);
    return data;
};

export const createEvolutionCheckinApi = async (checkinData) => {
    const { data } = await axios.post('/timeline/api/evolution-checkins', checkinData);
    return data;
};

export const deleteEvolutionCheckinApi = async (id) => {
    const { data } = await axios.delete(`/timeline/api/evolution-checkins/${id}`);
    return data;
};

import axios from "../../axios.config";

export const getTeamMembersApi = async () => {
    const { data } = await axios.get('/workout/api/team-members');
    return data;
};

export const getTeamMemberApi = async (personId) => {
    const { data } = await axios.get(`/workout/api/team-members/${personId}`);
    return data;
};

export const sendTeamMemberRequestApi = async (personId) => {
    const { data } = await axios.put(`/workout/api/team-members/request/${personId}`);
    return data;
};

export const acceptTeamMemberRequestApi = async (businessProfileId) => {
    const { data } = await axios.put(`/workout/api/team-members/accept/${businessProfileId}`);
    return data;
};

export const denyTeamMemberRequestApi = async (businessProfileId) => {
    const { data } = await axios.put(`/workout/api/team-members/deny/${businessProfileId}`);
    return data;
};

export const cancelTeamMemberRequestApi = async (personId) => {
    const { data } = await axios.put(`/workout/api/team-members/cancel/${personId}`);
    return data;
};
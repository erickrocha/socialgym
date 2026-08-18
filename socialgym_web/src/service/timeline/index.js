import axios from '../../axios.config';
import { uploadFileToS3 } from '../s3/index.js';

export const fetchFeedPosts = async ({ page = 0 } = {}) => {
    const { data } = await axios.get('/timeline/api/feed', {
        params: { page },
    });
    return Array.isArray(data) ? data : [];
};

export const addReactionToPost = async (postId, reactionType = 'Like') => {
    await axios.post(`/timeline/api/posts/${postId}/reactions`, { reactionType });
};

export const addCommentToPost = async (postId, content, authorAvatarUrl) => {
    const payload = {
        postId,
        content,
        ...(authorAvatarUrl ? { authorAvatarUrl } : {}),
    };
    const { data } = await axios.post(`/timeline/api/posts/${postId}/comments`, payload);
    return data;
};

const uploadMediaFiles = async (files = []) => {
    const uploadedMedia = [];

    for (const file of files) {
        const mediaType = file.type.startsWith('video/') ? 'Video' : 'Image';

        const { data } = await axios.get('/api/media/upload', {
            params: {
                mediaType,
                format: file.type,
                album: 'post',
            },
        });

        await uploadFileToS3(file, data.url);

        const uri = new URL(data.url);
        const publicUrl = `${uri.protocol}//${uri.host}${uri.pathname}`;

        uploadedMedia.push({
            mediaType,
            objectKey: data.objectKey,
            url: publicUrl,
        });
    }

    return uploadedMedia;
};

export const createFeedPost = async ({ content, files = [] }) => {
    const media = await uploadMediaFiles(files);

    const payload = {
        content: content?.trim() || '',
        ...(media.length ? { media } : {}),
    };

    const { data } = await axios.post('/timeline/api/posts', payload);
    return data;
};

export const fetchWorkoutSessions = async ({ startDate, endDate }) => {
    const { data } = await axios.get('/timeline/api/workout-sessions', {
        params: {
            startDate,
            endDate,
        },
    });

    return Array.isArray(data) ? data : [];
};

export const fetchEvolutionCheckins = async ({ startDate, endDate }) => {
    const { data } = await axios.get('/timeline/api/evolution-checkin', {
        params: {
            startDate,
            endDate,
        },
    });

    return Array.isArray(data) ? data : [];
};

export const createEvolutionCheckin = async (payload) => {
    const { data } = await axios.post('/timeline/api/evolution-checkin', payload);
    return data;
};

export const fetchFriendProfile = async (personId) => {
    const { data } = await axios.get(`/api/friends/${personId}`);
    return data;
};
